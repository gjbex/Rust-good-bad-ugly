cmake_minimum_required(VERSION 3.16)

project(julia_set_cpp VERSION 0.1.0 LANGUAGES CXX)

add_executable(julia-set-cpp
    src/main.cpp
)

target_compile_features(julia-set-cpp PRIVATE cxx_std_17)

if(MSVC)
    target_compile_options(julia-set-cpp PRIVATE /W4)
else()
    target_compile_options(julia-set-cpp PRIVATE -Wall -Wextra -Wpedantic)
endif()
