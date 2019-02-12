#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

void saxpy(int n, float a, float * restrict x, float * restrict y);

int main(int argc, char* argv[]){
    long size;
    if (argc == 2){
        size = atoi(argv[1]);
    } else {
        size = 32;
    }
    float f = 2.0;
    float* x = malloc(sizeof(float)*size);
    float* y = malloc(sizeof(float)*size);
    struct timeval start, end;
    printf("Running saxpy on arrays of size %d...\n", size);

    gettimeofday(&start, NULL);
    saxpy(size, f, x, y);
    gettimeofday(&end, NULL);

    int delta = end.tv_sec-start.tv_sec;
    int milli = end.tv_usec - start.tv_usec;

    printf("Finished saxpy. Time taken: %d, %d\n", delta, milli);
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
