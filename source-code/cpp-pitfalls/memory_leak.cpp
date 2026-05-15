#include <iostream>

int main() {
    int* values = new int[1024];

    values[0] = 42;
    std::cout << "first value: " << values[0] << '\n';

    values = nullptr;

    return 0;
}
