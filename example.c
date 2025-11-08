#include <stdio.h>

// Global variable declarations
int global_var = 10;
float global_float = 3.14;
char global_char = 65;


int add(int a, int b) {
    return a + b;
}

int main() {
    int a = 10;
    int b = 20;
    int c = add(a, b);
    printf("c = %d\n", c);
    return 0;
}