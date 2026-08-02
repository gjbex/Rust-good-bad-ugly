#include "interpolator.hpp"

#include <algorithm>
#include <cmath>
#include <functional>
#include <stdexcept>
#include <utility>

namespace interpolation {

LinearInterpolator::LinearInterpolator(
    std::vector<double> coordinates,
    std::vector<double> values
)
    : coordinates_(std::move(coordinates)), values_(std::move(values)) {
    if (coordinates_.size() != values_.size() || coordinates_.size() < 2) {
        throw std::invalid_argument("interpolation requires at least two coordinate-value pairs");
    }
    if (!std::all_of(coordinates_.begin(), coordinates_.end(), [](double value) {
            return std::isfinite(value);
        }) ||
        !std::all_of(values_.begin(), values_.end(), [](double value) {
            return std::isfinite(value);
        })) {
        throw std::invalid_argument("coordinates and values must be finite");
    }
    if (!std::is_sorted(coordinates_.begin(), coordinates_.end(), std::less<double>()) ||
        std::adjacent_find(coordinates_.begin(), coordinates_.end()) != coordinates_.end()) {
        throw std::invalid_argument("coordinates must be strictly increasing");
    }
}

double LinearInterpolator::evaluate(double coordinate) const {
    if (!std::isfinite(coordinate)) {
        throw std::invalid_argument("query coordinate must be finite");
    }
    if (coordinate < coordinates_.front() || coordinate > coordinates_.back()) {
        throw std::out_of_range("query coordinate lies outside the interpolation domain");
    }
    if (coordinate == coordinates_.back()) {
        return values_.back();
    }

    const auto upper = std::upper_bound(coordinates_.begin(), coordinates_.end(), coordinate);
    const auto upper_index = static_cast<std::size_t>(upper - coordinates_.begin());
    const auto lower_index = upper_index - 1;
    const double fraction =
        (coordinate - coordinates_[lower_index]) /
        (coordinates_[upper_index] - coordinates_[lower_index]);
    return values_[lower_index] + fraction * (values_[upper_index] - values_[lower_index]);
}

}  // namespace interpolation
