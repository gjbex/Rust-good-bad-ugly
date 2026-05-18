#include <iomanip>
#include <iostream>
#include <type_traits>

float add_float(float left, float right) {
    return left + right;
}

double add_double(double left, double right) {
    return left + right;
}

int main() {
    float value = 0.1f;

    auto promoted = value + 0.2;
    auto not_promoted = value + 0.2f;

    static_assert(std::is_same_v<decltype(promoted), double>);
    static_assert(std::is_same_v<decltype(not_promoted), float>);

    std::cout << std::setprecision(20);
    std::cout << "float + double literal has type double: " << promoted << '\n';
    std::cout << "float + float literal has type float:   " << not_promoted << '\n';
    std::cout << "function taking float:                  " << add_float(value, 0.2f) << '\n';
    std::cout << "function taking double:                 " << add_double(value, 0.2) << '\n';

    return 0;
}
