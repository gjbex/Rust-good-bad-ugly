#include <iostream>
#include <thread>
#include <vector>

int shared_counter = 0;

void increment_counter(int iterations) {
    for (int i = 0; i < iterations; ++i) {
        ++shared_counter;
    }
}

int main() {
    constexpr int thread_count = 4;
    constexpr int iterations_per_thread = 100000;

    std::vector<std::thread> workers;
    workers.reserve(thread_count);

    for (int i = 0; i < thread_count; ++i) {
        workers.emplace_back(increment_counter, iterations_per_thread);
    }

    for (auto& worker : workers) {
        worker.join();
    }

    std::cout << "expected: " << thread_count * iterations_per_thread << '\n';
    std::cout << "actual:   " << shared_counter << '\n';

    return 0;
}
