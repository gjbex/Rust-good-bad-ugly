#include <iostream>

int main() {
    int values[4] = {0, 1, 2, 3};

    for (int i = 0; i <= 4; ++i) {
        values[i] = i * 10;
    }

    std::cout << "first value: " << values[0] << '\n';

    return 0;
}
