use std::{
    error::Error,
    ffi::c_void,
    fmt,
    ptr::NonNull,
    slice,
    sync::{Mutex, MutexGuard, PoisonError},
};

use fftw_sys::{
    FFTW_ESTIMATE, fftw_alloc_complex, fftw_alloc_real, fftw_destroy_plan, fftw_execute, fftw_free,
    fftw_plan_dft_c2r_1d, fftw_plan_dft_r2c_1d, fftw_plan_s,
};
use num_complex::Complex64;

static FFTW_PLANNER_LOCK: Mutex<()> = Mutex::new(());

fn lock_planner() -> MutexGuard<'static, ()> {
    FFTW_PLANNER_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

#[derive(Debug, PartialEq, Eq)]
pub enum FftError {
    EmptyInput,
    LengthTooLarge(usize),
    AllocationFailed(&'static str),
    PlanCreationFailed(&'static str),
    LengthMismatch { expected: usize, actual: usize },
}

impl fmt::Display for FftError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "FFT length must be positive"),
            Self::LengthTooLarge(length) => {
                write!(formatter, "FFT length {length} does not fit in a C int")
            }
            Self::AllocationFailed(buffer) => {
                write!(formatter, "FFTW could not allocate the {buffer} buffer")
            }
            Self::PlanCreationFailed(direction) => {
                write!(formatter, "FFTW could not create the {direction} plan")
            }
            Self::LengthMismatch { expected, actual } => {
                write!(
                    formatter,
                    "expected {expected} values at the FFI boundary, received {actual}"
                )
            }
        }
    }
}

impl Error for FftError {}

struct FftwBuffer<T> {
    pointer: NonNull<T>,
    length: usize,
}

impl FftwBuffer<f64> {
    fn new_real(length: usize) -> Result<Self, FftError> {
        // SAFETY: FFTW accepts any element count and returns either an aligned
        // allocation or a null pointer. Ownership is transferred to this type.
        let pointer = unsafe { fftw_alloc_real(length) };
        let pointer = NonNull::new(pointer).ok_or(FftError::AllocationFailed("real-valued"))?;
        // SAFETY: The allocation contains `length` f64 values, and all-zero is
        // a valid bit pattern for f64.
        unsafe { pointer.as_ptr().write_bytes(0, length) };
        Ok(Self { pointer, length })
    }
}

impl FftwBuffer<Complex64> {
    fn new_complex(length: usize) -> Result<Self, FftError> {
        // SAFETY: The same allocation contract applies to fftw_alloc_complex.
        let pointer = unsafe { fftw_alloc_complex(length) };
        let pointer = NonNull::new(pointer).ok_or(FftError::AllocationFailed("complex-valued"))?;
        // SAFETY: Complex64 contains two f64 values, for which all-zero is a
        // valid bit pattern.
        unsafe { pointer.as_ptr().write_bytes(0, length) };
        Ok(Self { pointer, length })
    }
}

impl<T> FftwBuffer<T> {
    fn as_mut_pointer(&mut self) -> *mut T {
        self.pointer.as_ptr()
    }

    fn as_slice(&self) -> &[T] {
        // SAFETY: FFTW allocated space for exactly `length` initialized values.
        // The buffer remains alive and is not mutated for this borrow.
        unsafe { slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: This type owns the allocation, and the mutable borrow prevents
        // another Rust reference from accessing it concurrently.
        unsafe { slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length) }
    }
}

impl<T> Drop for FftwBuffer<T> {
    fn drop(&mut self) {
        // SAFETY: The pointer came from an FFTW allocation function and is
        // released exactly once when its owning buffer is dropped.
        unsafe { fftw_free(self.pointer.as_ptr().cast::<c_void>()) };
    }
}

struct FftwPlan(NonNull<fftw_plan_s>);

impl FftwPlan {
    fn forward(length: i32, input: *mut f64, output: *mut Complex64) -> Result<Self, FftError> {
        // SAFETY: Both pointers refer to FFTW-aligned buffers of the sizes
        // required for a real-to-complex transform of `length` values. The
        // process-wide lock serializes FFTW planner operations.
        let _planner_guard = lock_planner();
        let plan = unsafe { fftw_plan_dft_r2c_1d(length, input, output, FFTW_ESTIMATE) };
        NonNull::new(plan)
            .map(Self)
            .ok_or(FftError::PlanCreationFailed("forward"))
    }

    fn inverse(length: i32, input: *mut Complex64, output: *mut f64) -> Result<Self, FftError> {
        // SAFETY: Both pointers refer to FFTW-aligned buffers of the sizes
        // required for a complex-to-real transform of `length` values. The
        // process-wide lock serializes FFTW planner operations.
        let _planner_guard = lock_planner();
        let plan = unsafe { fftw_plan_dft_c2r_1d(length, input, output, FFTW_ESTIMATE) };
        NonNull::new(plan)
            .map(Self)
            .ok_or(FftError::PlanCreationFailed("inverse"))
    }

    fn execute(&mut self) {
        // SAFETY: The plan remains valid because it is owned by this value, and
        // the buffers bound during plan creation outlive it.
        unsafe { fftw_execute(self.0.as_ptr()) };
    }
}

impl Drop for FftwPlan {
    fn drop(&mut self) {
        // SAFETY: The pointer is a live FFTW plan and is destroyed exactly once.
        let _planner_guard = lock_planner();
        unsafe { fftw_destroy_plan(self.0.as_ptr()) };
    }
}

pub struct RealFft {
    length: usize,
    spectrum_length: usize,
    // Plans are declared before buffers so Rust destroys them before the
    // allocations whose addresses they retain.
    forward_plan: FftwPlan,
    inverse_plan: FftwPlan,
    real_buffer: FftwBuffer<f64>,
    complex_buffer: FftwBuffer<Complex64>,
}

impl RealFft {
    pub fn new(length: usize) -> Result<Self, FftError> {
        if length == 0 {
            return Err(FftError::EmptyInput);
        }
        let ffi_length = i32::try_from(length).map_err(|_| FftError::LengthTooLarge(length))?;
        let spectrum_length = length / 2 + 1;

        let mut real_buffer = FftwBuffer::new_real(length)?;
        let mut complex_buffer = FftwBuffer::new_complex(spectrum_length)?;

        let forward_plan = FftwPlan::forward(
            ffi_length,
            real_buffer.as_mut_pointer(),
            complex_buffer.as_mut_pointer(),
        )?;
        let inverse_plan = FftwPlan::inverse(
            ffi_length,
            complex_buffer.as_mut_pointer(),
            real_buffer.as_mut_pointer(),
        )?;

        Ok(Self {
            length,
            spectrum_length,
            forward_plan,
            inverse_plan,
            real_buffer,
            complex_buffer,
        })
    }

    pub fn spectrum_len(&self) -> usize {
        self.spectrum_length
    }

    pub fn forward(&mut self, signal: &[f64]) -> Result<Vec<Complex64>, FftError> {
        self.check_length(self.length, signal.len())?;
        self.real_buffer.as_mut_slice().copy_from_slice(signal);
        self.forward_plan.execute();
        Ok(self.complex_buffer.as_slice().to_vec())
    }

    pub fn inverse(&mut self, spectrum: &[Complex64]) -> Result<Vec<f64>, FftError> {
        self.check_length(self.spectrum_length, spectrum.len())?;
        self.complex_buffer.as_mut_slice().copy_from_slice(spectrum);
        self.inverse_plan.execute();

        let normalization = self.length as f64;
        Ok(self
            .real_buffer
            .as_slice()
            .iter()
            .map(|value| value / normalization)
            .collect())
    }

    fn check_length(&self, expected: usize, actual: usize) -> Result<(), FftError> {
        if actual == expected {
            Ok(())
        } else {
            Err(FftError::LengthMismatch { expected, actual })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_an_empty_transform() {
        assert!(matches!(RealFft::new(0), Err(FftError::EmptyInput)));
    }

    #[test]
    fn validates_slice_lengths_before_calling_fftw() {
        let mut transform = RealFft::new(8).unwrap();

        assert_eq!(
            transform.forward(&[0.0; 7]).unwrap_err(),
            FftError::LengthMismatch {
                expected: 8,
                actual: 7,
            }
        );
        assert_eq!(
            transform
                .inverse(&[Complex64::new(0.0, 0.0); 4])
                .unwrap_err(),
            FftError::LengthMismatch {
                expected: 5,
                actual: 4,
            }
        );
    }
}
