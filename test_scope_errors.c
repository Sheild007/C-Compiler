#include <stdio.h>

int global_var = 10;

int add(int a, int b)
{
    return a + b;
}

int main()
{
    int a = 10;
    int b = 20;
    int a = 30;                    // Error: variable redefinition in same scope
    int c = add(a, undefined_var); // Error: undefined variable
    unknown_function();            // Error: undefined function call
    printf("c = %d\n", c);
    return 0;
}

int add(int x, int y)
{ // Error: function redefinition
    return x + y;
}