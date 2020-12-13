cmake_minimum_required(VERSION 3.17)
project(chess)

set(CMAKE_CXX_STANDARD 20)

add_executable(chess main.cpp models/Pieces.h models/Positions.h models/King.h models/Table.h)