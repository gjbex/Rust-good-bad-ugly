#include "matrix.h"

#include <complex>
#include <cstddef>
#include <cctype>
#include <cstdlib>
#include <exception>
#include <iostream>
#include <limits>
#include <sstream>
#include <stdexcept>
#include <string>

struct Args {
    std::size_t max_iterations = 1000;
    std::size_t width = 800;
    std::size_t height = 600;
    double c_real = -0.5125;
    double c_imag = 0.5213;
};

void print_usage(const char* program)
{
    std::cout
        << "Usage: " << program << " [OPTIONS]\n\n"
        << "Options:\n"
        << "  -m, --max-iterations <N>  Maximum number of iterations [default: 1000]\n"
        << "  -x, --width <N>           Output width [default: 800]\n"
        << "  -y, --height <N>          Output height [default: 600]\n"
        << "  -r, --c-real <VALUE>      Real component of c [default: -0.5125]\n"
        << "  -i, --c-imag <VALUE>      Imaginary component of c [default: 0.5213]\n"
        << "  -h, --help                Print help\n";
}

std::string option_value(int& index, int argc, char* argv[], const std::string& option)
{
    const std::string argument = argv[index];
    const std::string prefix = option + "=";
    if (argument.rfind(prefix, 0) == 0) {
        return argument.substr(prefix.size());
    }
    if (index + 1 >= argc) {
        throw std::invalid_argument("missing value for " + option);
    }
    ++index;
    return argv[index];
}

std::size_t parse_size(const std::string& value, const std::string& option)
{
    if (value.empty()) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    for (const unsigned char character : value) {
        if (std::isdigit(character) == 0) {
            throw std::invalid_argument("invalid integer for " + option + ": " + value);
        }
    }

    std::size_t parsed_chars = 0;
    unsigned long long parsed = 0;
    try {
        parsed = std::stoull(value, &parsed_chars);
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    if (parsed_chars != value.size() || parsed > std::numeric_limits<std::size_t>::max()) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    return static_cast<std::size_t>(parsed);
}

double parse_double(const std::string& value, const std::string& option)
{
    std::size_t parsed_chars = 0;
    double parsed = 0.0;
    try {
        parsed = std::stod(value, &parsed_chars);
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid floating-point value for " + option + ": " + value);
    }
    if (parsed_chars != value.size()) {
        throw std::invalid_argument("invalid floating-point value for " + option + ": " + value);
    }
    return parsed;
}

Args parse_args(int argc, char* argv[])
{
    Args args;
    for (int index = 1; index < argc; ++index) {
        const std::string argument = argv[index];
        if (argument == "-h" || argument == "--help") {
            print_usage(argv[0]);
            std::exit(0);
        } else if (argument == "-m" || argument == "--max-iterations"
            || argument.rfind("--max-iterations=", 0) == 0) {
            args.max_iterations = parse_size(option_value(index, argc, argv, "--max-iterations"),
                "--max-iterations");
        } else if (argument == "-x" || argument == "--width" || argument.rfind("--width=", 0) == 0) {
            args.width = parse_size(option_value(index, argc, argv, "--width"), "--width");
        } else if (argument == "-y" || argument == "--height" || argument.rfind("--height=", 0) == 0) {
            args.height = parse_size(option_value(index, argc, argv, "--height"), "--height");
        } else if (argument == "-r" || argument == "--c-real" || argument.rfind("--c-real=", 0) == 0) {
            args.c_real = parse_double(option_value(index, argc, argv, "--c-real"), "--c-real");
        } else if (argument == "-i" || argument == "--c-imag" || argument.rfind("--c-imag=", 0) == 0) {
            args.c_imag = parse_double(option_value(index, argc, argv, "--c-imag"), "--c-imag");
        } else {
            throw std::invalid_argument("unknown option: " + argument);
        }
    }
    return args;
}

Matrix<std::complex<double>> initialize_z(std::size_t rows, std::size_t cols)
{
    Matrix<std::complex<double>> z(rows, cols, std::complex<double>{0.0, 0.0});
    constexpr double domain_min = -2.0;
    constexpr double domain_max = 2.0;
    const double delta_re = (domain_max - domain_min) / static_cast<double>(cols);
    const double delta_im = (domain_max - domain_min) / static_cast<double>(rows);

    for (std::size_t row = 0; row < rows; ++row) {
        for (std::size_t col = 0; col < cols; ++col) {
            z.set(row, col, std::complex<double>{
                                domain_min + static_cast<double>(col) * delta_re,
                                domain_min + static_cast<double>(row) * delta_im,
                            });
        }
    }
    return z;
}

std::size_t iterate_z_value(
    std::complex<double> z, std::complex<double> c, std::size_t max_iterations)
{
    auto z_n = z;
    for (std::size_t n = 0; n < max_iterations; ++n) {
        if (std::abs(z_n) > 2.0) {
            return n;
        }
        z_n = z_n * z_n + c;
    }
    return max_iterations;
}

Matrix<std::size_t> iterate_z_matrix(
    const Matrix<std::complex<double>>& z, std::complex<double> c, std::size_t max_iterations)
{
    Matrix<std::size_t> result(z.rows(), z.cols(), 0);
    for (std::size_t row = 0; row < z.rows(); ++row) {
        for (std::size_t col = 0; col < z.cols(); ++col) {
            result.set(row, col, iterate_z_value(z.get(row, col), c, max_iterations));
        }
    }
    return result;
}

int main(int argc, char* argv[])
{
    try {
        const Args args = parse_args(argc, argv);
        const auto c = std::complex<double>{args.c_real, args.c_imag};
        const auto z = initialize_z(args.height, args.width);
        const auto result = iterate_z_matrix(z, c, args.max_iterations);

        for (std::size_t row = 0; row < result.rows(); ++row) {
            for (std::size_t col = 0; col < result.cols(); ++col) {
                std::cout.width(3);
                std::cout << result.get(row, col) << ' ';
            }
            std::cout << '\n';
        }
    } catch (const std::exception& error) {
        std::cerr << "error: " << error.what() << '\n';
        return 1;
    }
    return 0;
}
