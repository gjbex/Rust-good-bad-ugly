#include <iostream>

int main() {
    int* value = new int{12};

    std::cout << "allocated value: " << *value << '\n';
    delete value;
    delete value;

    return 0;
}
