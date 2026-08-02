#ifndef INTERPOLATOR_C_H
#define INTERPOLATOR_C_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct interpolation_handle interpolation_handle;

enum interpolation_status {
    INTERPOLATION_SUCCESS = 0,
    INTERPOLATION_NULL_POINTER = 1,
    INTERPOLATION_INVALID_LENGTH = 2,
    INTERPOLATION_NON_FINITE_VALUE = 3,
    INTERPOLATION_NON_INCREASING_COORDINATES = 4,
    INTERPOLATION_OUT_OF_RANGE = 5,
    INTERPOLATION_ALLOCATION_FAILED = 6,
    INTERPOLATION_UNKNOWN_ERROR = 7
};

int interpolation_create(
    const double* coordinates,
    size_t coordinate_count,
    const double* values,
    size_t value_count,
    interpolation_handle** output
);

int interpolation_evaluate(
    const interpolation_handle* handle,
    double coordinate,
    double* output
);

void interpolation_destroy(interpolation_handle* handle);

#ifdef __cplusplus
}
#endif

#endif

