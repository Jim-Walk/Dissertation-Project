#include <stdio.h>

void add_four(int * foo){
    if(!foo){
        return;
    }
    *foo = *foo + 4;
}

void main(){
    int *a, var;
    var = 2;
    a = &var;
    add_four(a);
    printf("%d\n", *a);
}
