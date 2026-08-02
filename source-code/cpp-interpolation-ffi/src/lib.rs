mod raw;

use std::{error::Error, fmt, ptr::NonNull};

#[derive(Debug, PartialEq)]
pub enum InterpolationError {
    InvalidLengths {
        coordinate_count: usize,
        value_count: usize,
    },
    NonFiniteData,
    NonIncreasingCoordinates,
    NonFiniteQuery,
    QueryOutOfRange {
        query: f64,
        minimum: f64,
        maximum: f64,
    },
    AllocationFailed,
    NativeContractViolation(i32),
}

impl fmt::Display for InterpolationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLengths {
                coordinate_count,
                value_count,
            } => write!(
                formatter,
                "interpolation requires equal coordinate and value lengths of at least two; received {coordinate_count} and {value_count}"
            ),
            Self::NonFiniteData => write!(formatter, "coordinates and values must be finite"),
            Self::NonIncreasingCoordinates => {
                write!(formatter, "coordinates must be strictly increasing")
            }
            Self::NonFiniteQuery => write!(formatter, "query coordinate must be finite"),
            Self::QueryOutOfRange {
                query,
                minimum,
                maximum,
            } => write!(
                formatter,
                "query coordinate {query} lies outside [{minimum}, {maximum}]"
            ),
            Self::AllocationFailed => write!(formatter, "C++ could not allocate the interpolator"),
            Self::NativeContractViolation(status) => {
                write!(
                    formatter,
                    "C interpolation facade returned unexpected status {status}"
                )
            }
        }
    }
}

impl Error for InterpolationError {}

#[derive(Debug)]
pub struct LinearInterpolator {
    handle: NonNull<raw::InterpolationHandle>,
    minimum: f64,
    maximum: f64,
}

impl LinearInterpolator {
    pub fn new(coordinates: &[f64], values: &[f64]) -> Result<Self, InterpolationError> {
        let mut handle = std::ptr::null_mut();
        // SAFETY: Both slices provide valid pointers for their stated lengths.
        // The C facade copies their values and writes at most one handle pointer.
        let status = unsafe {
            raw::interpolation_create(
                coordinates.as_ptr(),
                coordinates.len(),
                values.as_ptr(),
                values.len(),
                &mut handle,
            )
        };
        if status != raw::SUCCESS {
            return Err(Self::creation_error(
                status,
                coordinates.len(),
                values.len(),
            ));
        }

        let handle = NonNull::new(handle).ok_or(InterpolationError::NativeContractViolation(
            raw::NULL_POINTER,
        ))?;
        Ok(Self {
            handle,
            minimum: coordinates[0],
            maximum: coordinates[coordinates.len() - 1],
        })
    }

    pub fn evaluate(&self, query: f64) -> Result<f64, InterpolationError> {
        let mut value = 0.0;
        // SAFETY: `handle` is owned by `self` and remains live for this call.
        // The output points to one writable `f64`.
        let status =
            unsafe { raw::interpolation_evaluate(self.handle.as_ptr(), query, &mut value) };
        match status {
            raw::SUCCESS => Ok(value),
            raw::NON_FINITE_VALUE => Err(InterpolationError::NonFiniteQuery),
            raw::OUT_OF_RANGE => Err(InterpolationError::QueryOutOfRange {
                query,
                minimum: self.minimum,
                maximum: self.maximum,
            }),
            other => Err(InterpolationError::NativeContractViolation(other)),
        }
    }

    fn creation_error(
        status: i32,
        coordinate_count: usize,
        value_count: usize,
    ) -> InterpolationError {
        match status {
            raw::INVALID_LENGTH => InterpolationError::InvalidLengths {
                coordinate_count,
                value_count,
            },
            raw::NON_FINITE_VALUE => InterpolationError::NonFiniteData,
            raw::NON_INCREASING_COORDINATES => InterpolationError::NonIncreasingCoordinates,
            raw::ALLOCATION_FAILED => InterpolationError::AllocationFailed,
            other => InterpolationError::NativeContractViolation(other),
        }
    }
}

impl Drop for LinearInterpolator {
    fn drop(&mut self) {
        // SAFETY: The handle was returned by `interpolation_create`, is owned by
        // this value, and is released exactly once here.
        unsafe { raw::interpolation_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_between_points_and_at_endpoints() {
        let interpolator =
            LinearInterpolator::new(&[0.0, 1.0, 2.0, 4.0], &[0.0, 1.0, 4.0, 16.0]).unwrap();

        assert_eq!(interpolator.evaluate(0.0).unwrap(), 0.0);
        assert_eq!(interpolator.evaluate(1.5).unwrap(), 2.5);
        assert_eq!(interpolator.evaluate(4.0).unwrap(), 16.0);
    }

    #[test]
    fn rejects_invalid_input_data() {
        assert_eq!(
            LinearInterpolator::new(&[0.0, 1.0], &[0.0]).unwrap_err(),
            InterpolationError::InvalidLengths {
                coordinate_count: 2,
                value_count: 1,
            }
        );
        assert_eq!(
            LinearInterpolator::new(&[0.0], &[0.0]).unwrap_err(),
            InterpolationError::InvalidLengths {
                coordinate_count: 1,
                value_count: 1,
            }
        );
        assert_eq!(
            LinearInterpolator::new(&[0.0, f64::NAN], &[0.0, 1.0]).unwrap_err(),
            InterpolationError::NonFiniteData
        );
        assert_eq!(
            LinearInterpolator::new(&[0.0, 2.0, 1.0], &[0.0, 4.0, 1.0]).unwrap_err(),
            InterpolationError::NonIncreasingCoordinates
        );
    }

    #[test]
    fn rejects_invalid_queries() {
        let interpolator = LinearInterpolator::new(&[0.0, 1.0], &[2.0, 3.0]).unwrap();

        assert_eq!(
            interpolator.evaluate(f64::INFINITY).unwrap_err(),
            InterpolationError::NonFiniteQuery
        );
        assert_eq!(
            interpolator.evaluate(-0.5).unwrap_err(),
            InterpolationError::QueryOutOfRange {
                query: -0.5,
                minimum: 0.0,
                maximum: 1.0,
            }
        );
    }
}
