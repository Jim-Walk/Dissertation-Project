#!/usr/bin/python3

import sys
import time

def main():

    if len(sys.argv) > 1:
        size = int(sys.argv[1])
        print("size = ", size)
    else:
        size = 32

    x = [6.7] * size
    y = [9.8] * size
    f = 2.0

    start = time.time()
    saxpy(size, f, x, y)
    end = time.time()
    f_time = end - start
    print("Saxpy finished in ", f_time)


def saxpy(n, a, x, y):
    i = 0
    while i < a:
        y[i] = a*x[i] + y[i]
        i += 1

if __name__ == '__main__':
    main()
