cmake_minimum_required(VERSION 3.10)
project(maya)

include_directories(${CMAKE_SOURCE_DIR}/include)

add_executable(maya ${CMAKE_SOURCE_DIR}/src/main.c)