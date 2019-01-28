#include <stdio.h>
#include <stdlib.h>

void saxpy(int n, float a, float * restrict x, float * restrict y);

int main(int argc, char* argv[]){
    int size;
    if (argc == 2){
        size = atoi(argv[1]);
    } else {
        size = 32;
    }
    float f = 2.0;
    float* x = malloc(sizeof(float)*size);
    float* y = malloc(sizeof(float)*size);
    printf("Running saxpy on arrays of size %d...\n", size);

    
    saxpy(size, f, x, y);
    printf("Finished saxpy\n");
    return 1;
}


// This saxpy implentation uses the restrict keyword, which should mean that the 
// generated assembly code is optimised, as x and y must be pointers, the compiler
// will not reload the value in their addresses
void saxpy(int n, float a, float * restrict x, float * restrict y){
    
    for (int i=0; i<n; i++){
        y[i] = a*x[i] + y[i];   
    }

}
