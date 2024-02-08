#!/bin/bash

test_name=$(basename "$0" .sh)
t=out/tests/$test_name

mkdir -p "$t"

cat <<EOF | gcc -o "$t"/a.o -c -xc -
#include <stdio.h>

int mian(void) {
    printf("hello world\n");
    return 0;
}
EOF


target/debug/linker_rs "$t"/a.o