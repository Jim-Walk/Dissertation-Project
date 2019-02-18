#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define N 100000

void saxpy(int n, float a, float * restrict x, float * restrict y);

int main(int argc, char* argv[]){
    unsigned long size;
    if (argc == 2){
        size = atoi(argv[1]);
    } else {
        size = 32;
    }
    float f = 2.0;
    float* x = malloc(sizeof(float)*size);
    float* y = malloc(sizeof(float)*size);
    struct timespec start;
    struct timespec end;
    printf("Running saxpy on arrays of size %lu...\n", size);

    int i;
    timespec_get(&start, TIME_UTC);
    for (i=0; i<N; i++){
        saxpy(size, f, x, y);
    }
    timespec_get(&end, TIME_UTC);

    long final_s = (long)difftime(end.tv_sec, start.tv_sec);
    long final_m = end.tv_nsec - start.tv_nsec;

    long f_time = (final_s *1000) + (final_m / 1000000);

    printf("Finished saxpy. Time taken: %lu milliseconds\n", f_time);
    return 1;
}


// This saxpy implentation uses the restrict keyword, which should mean that the 
// generated assembly code is optimised, as x and y must be pointers, the compiler
// will not reload the value in their addresses
void saxpy(int n, float a, float * x, float * y){
    
    for (int i=0; i<n; i++){
        y[i] = a*x[i] + y[i];   
    }

}
