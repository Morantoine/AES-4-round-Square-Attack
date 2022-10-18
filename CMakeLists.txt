cmake_minimum_required(VERSION 3.22)
project(tp1_crypto C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

add_executable(tp1_crypto
        aes-128_enc.c
        aes-128_enc.h
        distinguisher.c)
