#include <stdio.h>

int global_var = 10;

int main()
{
    int a = 10;
    printf("Outer a = %d\n", a);

    {
        int a = 20; // This should be OK - shadows outer 'a'
        int b = 30;
        printf("Inner a = %d, b = %d\n", a, b);

        {
            int c = a + b; // Should be OK - accesses outer variables
            printf("c = %d\n", c);
        }

        // printf("c = %d\n", c);  // Would be error - c out of scope
    }

    printf("Outer a again = %d\n", a); // Should be OK
    // printf("b = %d\n", b);  // Would be error - b out of scope

    return 0;
}