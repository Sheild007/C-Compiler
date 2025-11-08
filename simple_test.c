// Simple test to verify parser works correctly
#include <stdio.h>

int global_var = 10;

int add(int a, int b) {
    int result = a + b;
    return result;
}

int main() {
    int x = 5;
    int y = 10;
    int sum = add(x, y);
    return 0;
}

