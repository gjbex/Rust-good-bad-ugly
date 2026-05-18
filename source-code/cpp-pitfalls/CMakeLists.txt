cmake_minimum_required(VERSION 3.16)

project(cpp_pitfall_examples LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(ENABLE_SANITIZERS "Build examples with AddressSanitizer and UndefinedBehaviorSanitizer" OFF)
option(ENABLE_THREAD_SANITIZER "Build the data-race example with ThreadSanitizer" OFF)

find_package(Threads REQUIRED)

add_executable(use_after_free use_after_free.cpp)
add_executable(dangling_pointer dangling_pointer.cpp)
add_executable(double_free double_free.cpp)
add_executable(data_race data_race.cpp)
add_executable(buffer_overflow buffer_overflow.cpp)
add_executable(uninitialized_read uninitialized_read.cpp)
add_executable(memory_leak memory_leak.cpp)
add_executable(double_promotion double_promotion.cpp)

target_link_libraries(data_race PRIVATE Threads::Threads)

foreach(target
        use_after_free
        dangling_pointer
        double_free
        buffer_overflow
        uninitialized_read
        memory_leak
        double_promotion)
    target_compile_options(${target} PRIVATE -Wall -Wextra -pedantic)
    if(ENABLE_SANITIZERS)
        target_compile_options(${target} PRIVATE -fsanitize=address,undefined -fno-omit-frame-pointer)
        target_link_options(${target} PRIVATE -fsanitize=address,undefined -fno-omit-frame-pointer)
    endif()
endforeach()

target_compile_options(data_race PRIVATE -Wall -Wextra -pedantic)
if(ENABLE_THREAD_SANITIZER)
    target_compile_options(data_race PRIVATE -fsanitize=thread -fno-omit-frame-pointer)
    target_link_options(data_race PRIVATE -fsanitize=thread -fno-omit-frame-pointer)
elseif(ENABLE_SANITIZERS)
    target_compile_options(data_race PRIVATE -fsanitize=undefined -fno-omit-frame-pointer)
    target_link_options(data_race PRIVATE -fsanitize=undefined -fno-omit-frame-pointer)
endif()
