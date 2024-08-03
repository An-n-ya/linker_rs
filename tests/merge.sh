#!/bin/bash

test_name=$(basename "$0" .sh)
t=out/tests/$test_name

mkdir -p "$t"

cat <<EOF | gcc -o "$t"/a.o -c -xc -
#include <stdio.h>

const int a = 1;
const char *s = "const string";
int main(void) {
    printf("hello world %d\n", a);
    printf("hello hello\n");
    hello();
    return 0;
}
EOF
cat <<EOF | gcc -o "$t"/b.o -c -xc -
#include <stdio.h>

const int a = 1;
const char *s = "const string";
int hello(void) {
    printf("hello world %d\n", a);
    printf("hello annya\n");
    return 0;
}
EOF


# target/debug/linker_rs "$t"/a.o
gcc -B. -fno-lto -static "$t"/a.o "$t"/a.o -o "$t"/out