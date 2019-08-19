#!/usr/bin/python3
import sys
import numpy as np
import matplotlib.pyplot as plt
import string

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res_y = []
    res_x = []
    i = 0
    t = str.maketrans('', '',string.punctuation)
    while i < len(data):
        s = data[i].split()
        if s:
            if s[1] == 'r5301c7':
                flops = int(s[0].translate(t))
                flops += 2 * int(data[i+2].split()[0].translate(t))
                res_x += [flops/16630000000]
                flops = flops / float(data[i+7].split()[0])
                res_y += [flops]
        i+=1

    return res_x, res_y

# Takes a list of times and calculates speedup
# speedup = t1/tn
def get_speedup(res):
    i = 0
    t1 = res[0]
    while i < len(res):
        res[i] = t1/res[i]
        i += 1

def abline(slope, intercept):
    axes = plt.gca()
    x_vals = np.array(axes.get_xlim())
    y_vals = intercept + slope * x_vals
    return x_vals, y_vals

def flops_per_byte(res):
    flby = []
    for r in res:
        flby += [r/16630000000]
    return flby

if __name__ == '__main__':
    labels = []
    all_res_x = []
    all_res_y = []
    i = 1
    while i < len(sys.argv):
        labels += [sys.argv[i].split('.')[0]]
        x, y = parse_file(sys.argv[i])
        all_res_x += [x]
        all_res_y += [y]
        i += 1


    markers = ["o", "v", "s", "^", "x", "1", "2", "3", "4"]
    threads = [1,2] + list(range(4,37,4))
    j = 0
    fig, ax = plt.subplots()
    while j < len(all_res_x):
        print(labels[j])
        print('Threads, Flops,\tOperation Intensity')
        for i in range(len(all_res_x[j])):
            print(threads[i],'\t', all_res_y[j][i], all_res_x[j][i])
        ax.plot(all_res_x[j],all_res_y[j],label=labels[j], marker=markers[j])
        j += 1
    ax.set_xlim(2,15)
    peak_band_x, peak_band_y = abline(135106598000, 0)
    print(peak_band_x, peak_band_y)
    ax.plot(peak_band_x, peak_band_y, label="Peak Bandwidth")
    ax.plot(peak_band_x, [33782000000 for i in peak_band_x], label="Maximum Flop/s")
    t = 'Sparse Matrix Multiplication - Roofline'
    ax.set(xlabel='Operational Intensity (MFlops/MByte)', ylabel='Attainable MFlop/s',title=t)
    ax.set_yscale('log')
    #ax.set_xscale('log')
    ax.legend()
    fig.savefig('roofline2.png')
