#!/bin/bash

mkdir -p build
mkdir -p bin

nasm -o ./build/hello.o -f elf64 *.asm
ld ./build/*.o -o ./bin/hello
./bin/hello
echo $?
