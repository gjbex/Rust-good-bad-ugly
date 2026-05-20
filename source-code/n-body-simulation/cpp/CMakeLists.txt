cmake_minimum_required(VERSION 3.16)

project(n_body_simulation_cpp VERSION 0.1.0 LANGUAGES CXX)

add_executable(n-body-simulation-cpp
    src/main.cpp
    src/system.cpp
)

target_compile_features(n-body-simulation-cpp PRIVATE cxx_std_17)

if(MSVC)
    target_compile_options(n-body-simulation-cpp PRIVATE /W4)
else()
    target_compile_options(n-body-simulation-cpp PRIVATE -Wall -Wextra -Wpedantic)
endif()
