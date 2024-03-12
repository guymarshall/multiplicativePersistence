#!/bin/bash

if [ ! -d "target" ]; then
    mkdir target
fi

nvcc main.cu -o main

if [ $? -eq 0 ]; then
    echo "Compilation successful"

    mv main target

    cd target

    ./main
else
    echo "Compilation failed"
fi