use std::os::raw::{c_double, c_int};

pub const SUCCESS: c_int = 0;
pub const NULL_POINTER: c_int = 1;
pub const INVALID_LENGTH: c_int = 2;
pub const NON_FINITE_VALUE: c_int = 3;
pub const NON_INCREASING_COORDINATES: c_int = 4;
pub const OUT_OF_RANGE: c_int = 5;
pub const ALLOCATION_FAILED: c_int = 6;
pub const UNKNOWN_ERROR: c_int = 7;
#[repr(C)]
pub struct InterpolationHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn interpolation_create(
        coordinates: *const c_double,
        coordinate_count: usize,
        values: *const c_double,
        value_count: usize,
        output: *mut *mut InterpolationHandle,
    ) -> c_int;

    pub fn interpolation_evaluate(
        handle: *const InterpolationHandle,
        coordinate: c_double,
        output: *mut c_double,
    ) -> c_int;

    pub fn interpolation_destroy(handle: *mut InterpolationHandle);
}
