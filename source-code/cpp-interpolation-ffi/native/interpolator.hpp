#ifndef INTERPOLATOR_HPP
#define INTERPOLATOR_HPP

#include <vector>

namespace interpolation {

class LinearInterpolator {
public:
    LinearInterpolator(std::vector<double> coordinates, std::vector<double> values);

    [[nodiscard]] double evaluate(double coordinate) const;

private:
    std::vector<double> coordinates_;
    std::vector<double> values_;
};

}  // namespace interpolation

#endif

