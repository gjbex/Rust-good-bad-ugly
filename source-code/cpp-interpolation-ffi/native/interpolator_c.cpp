#include "interpolator_c.h"

#include "interpolator.hpp"

#include <algorithm>
#include <cmath>
#include <functional>
#include <new>
#include <stdexcept>
#include <utility>
#include <vector>

struct interpolation_handle {
    explicit interpolation_handle(interpolation::LinearInterpolator value)
        : interpolator(std::move(value)) {}

    interpolation::LinearInterpolator interpolator;
};

extern "C" int interpolation_create(
    const double* coordinates,
    size_t coordinate_count,
    const double* values,
    size_t value_count,
    interpolation_handle** output
) {
    if (output == nullptr) {
        return INTERPOLATION_NULL_POINTER;
    }
    *output = nullptr;

    if (coordinate_count != value_count || coordinate_count < 2) {
        return INTERPOLATION_INVALID_LENGTH;
    }
    if (coordinates == nullptr || values == nullptr) {
        return INTERPOLATION_NULL_POINTER;
    }
    if (!std::all_of(coordinates, coordinates + coordinate_count, [](double value) {
            return std::isfinite(value);
        }) ||
        !std::all_of(values, values + value_count, [](double value) {
            return std::isfinite(value);
        })) {
        return INTERPOLATION_NON_FINITE_VALUE;
    }
    if (!std::is_sorted(coordinates, coordinates + coordinate_count, std::less<double>()) ||
        std::adjacent_find(coordinates, coordinates + coordinate_count) !=
            coordinates + coordinate_count) {
        return INTERPOLATION_NON_INCREASING_COORDINATES;
    }

    try {
        std::vector<double> owned_coordinates(coordinates, coordinates + coordinate_count);
        std::vector<double> owned_values(values, values + value_count);
        auto interpolator = interpolation::LinearInterpolator(
            std::move(owned_coordinates),
            std::move(owned_values)
        );
        *output = new interpolation_handle(std::move(interpolator));
        return INTERPOLATION_SUCCESS;
    } catch (const std::bad_alloc&) {
        return INTERPOLATION_ALLOCATION_FAILED;
    } catch (...) {
        return INTERPOLATION_UNKNOWN_ERROR;
    }
}

extern "C" int interpolation_evaluate(
    const interpolation_handle* handle,
    double coordinate,
    double* output
) {
    if (handle == nullptr || output == nullptr) {
        return INTERPOLATION_NULL_POINTER;
    }
    if (!std::isfinite(coordinate)) {
        return INTERPOLATION_NON_FINITE_VALUE;
    }

    try {
        *output = handle->interpolator.evaluate(coordinate);
        return INTERPOLATION_SUCCESS;
    } catch (const std::out_of_range&) {
        return INTERPOLATION_OUT_OF_RANGE;
    } catch (...) {
        return INTERPOLATION_UNKNOWN_ERROR;
    }
}

extern "C" void interpolation_destroy(interpolation_handle* handle) {
    delete handle;
}
