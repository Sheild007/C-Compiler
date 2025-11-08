#include <stdio.h>

// Global variable declarations
int global_var = 10;
float global_float = 3.14;
char global_char = 65;

// Static variable
static int static_var = 20;

// Const variable
const double const_value = 2.718;

// Function declaration
int add(int a, int b);

// Simple function with return
int multiply(int x, int y) {
    return x * y;
}

// Function with multiple parameters
float calculate(float a, float b, float c) {
    float result = a + b - c;
    return result;
}

// Function with if-else
int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Function with while loop
int factorial(int n) {
    int result = 1;
    int i = 1;
    while (i <= n) {
        result = result * i;
        i = i + 1;
    }
    return result;
}

// Function with for loop
int sum_array(int size) {
    int sum = 0;
    int i;
    for (i = 0; i < size; i = i + 1) {
        sum = sum + i;
    }
    return sum;
}

// Function with nested loops and conditions
void print_pattern(int n) {
    int i;
    int j;
    for (i = 0; i < n; i = i + 1) {
        for (j = 0; j <= i; j = j + 1) {
            if (j % 2 == 0) {
                int x = 5;
            }
        }
    }
}

// Function with break statement
int find_first_even(int limit) {
    int i;
    for (i = 0; i < limit; i = i + 1) {
        if (i % 2 == 0) {
            break;
        }
    }
    return i;
}

// Function with complex expressions
int complex_calculation(int a, int b, int c) {
    int x = a + b * c;
    int y = (a + b) * c;
    int z = a - b / c;
    int w = a % b;
    
    if (x > y && y < z) {
        return x;
    } else {
        if (z != w) {
            return y;
        } else {
            return z;
        }
    }
}

// Function with logical operators
int logical_operations(int a, int b) {
    if (a > 0 && b > 0) {
        return 1;
    }
    if (a < 0 || b < 0) {
        return -1;
    }
    return 0;
}

// Function with bitwise operations
int bitwise_ops(int a, int b) {
    int and_result = a & b;
    int or_result = a | b;
    int xor_result = a ^ b;
    return and_result + or_result - xor_result;
}

// Function with comparison operators
int compare_values(int a, int b, int c) {
    if (a == b && b != c) {
        return 1;
    }
    if (a < b || b > c) {
        return 2;
    }
    if (a <= b && b >= c) {
        return 3;
    }
    return 0;
}

// Main function
int main() {
    int local_var = 5;
    float local_float = 1.5;
    char local_char = 66;
    
    // Variable assignments
    local_var = 10;
    local_float = 2.5;
    local_char = 67;
    
    // Function calls
    int result1 = multiply(3, 4);
    float result2 = calculate(1.0, 2.0, 3.0);
    int result3 = max(5, 10);
    
    // Expressions
    int sum = result1 + result2;
    int product = result1 * result3;
    int quotient = result3 / result1;
    
    // Complex control flow
    if (result1 > 5) {
        if (result2 > 0) {
            result3 = result1 + result2;
        } else {
            result3 = result1 - result2;
        }
    } else {
        result3 = result1 * 2;
    }
    
    // Loop with break
    int i;
    for (i = 0; i < 10; i = i + 1) {
        if (i > 5) {
            break;
        }
    }
    
    // While loop
    int count = 0;
    while (count < 10) {
        count = count + 1;
    }
    
    return 0;
}

