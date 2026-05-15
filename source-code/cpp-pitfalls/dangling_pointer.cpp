#include <iostream>

int* make_dangling_pointer() {
    int local_value = 7;
    return &local_value;
}

int main() {
    int* value = make_dangling_pointer();

    std::cout << "dangling value: " << *value << '\n';

    return 0;
}
