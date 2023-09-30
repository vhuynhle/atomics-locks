cmake_minimum_required(VERSION 3.22)
project(rust-atomics)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(MSVC)
  add_compile_options(/W4 /WX)
else()
  add_compile_options(-Wall -Wextra -pedantic -Werror)
endif()

add_subdirectory(ch02)
add_subdirectory(ch03)
add_subdirectory(ch04)
add_subdirectory(ch05)
