#include <stdio.h>

void main(){
    int *a, *b, var;
    var = 2;
    a = &var;
    printf("*a = %d\n", *a);
    b = a;
    printf("*b = %d\n", *b);
    *b = *b + 6;
    printf("*b = %d *a = %d\n", *b, *a);
}
