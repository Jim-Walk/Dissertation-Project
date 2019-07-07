Reference implementation of sparse has a bug. When the program is run with `./sparse 12 2 11 10` the program tries to write data to memory which has not been allocated.

This bug was discovered through porting the benchmark to Rust, which panics when you try to access an array index out of bounds. I confirmed the bug by adding the line

```
printf("nent: %llu elm: %llu \n", nent, elm+4);
```
after initilisation, and seeing that the ultimate thread writes to address 171966467 whilst the array col\_index is of length 171966464.

To ensure that the test is fair I will make sure my implementation does not write to these extra locations
