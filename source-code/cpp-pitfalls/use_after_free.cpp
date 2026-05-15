#include <iostream>

int main() {
    int* value = new int{42};

    std::cout << "before delete: " << *value << '\n';
    delete value;

    std::cout << "after delete: " << *value << '\n';

    return 0;
}
