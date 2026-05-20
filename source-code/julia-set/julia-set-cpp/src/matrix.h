#ifndef JULIA_SET_CPP_MATRIX_HPP
#define JULIA_SET_CPP_MATRIX_HPP

#include <cstddef>
#include <stdexcept>
#include <string>
#include <vector>

template <typename T>
class Matrix {
public:
    Matrix(std::size_t rows, std::size_t cols, const T& value)
        : rows_{rows}, cols_{cols}, data_(rows * cols, value)
    {
    }

    [[nodiscard]] std::size_t rows() const
    {
        return rows_;
    }

    [[nodiscard]] std::size_t cols() const
    {
        return cols_;
    }

    [[nodiscard]] const T& get(std::size_t row, std::size_t col) const
    {
        return data_.at(index(row, col));
    }

    void set(std::size_t row, std::size_t col, const T& value)
    {
        data_.at(index(row, col)) = value;
    }

private:
    [[nodiscard]] std::size_t index(std::size_t row, std::size_t col) const
    {
        if (row >= rows_ || col >= cols_) {
            throw std::out_of_range(
                "matrix index (" + std::to_string(row) + ", " + std::to_string(col)
                + ") is out of bounds");
        }
        return row * cols_ + col;
    }

    std::size_t rows_;
    std::size_t cols_;
    std::vector<T> data_;
};

#endif
