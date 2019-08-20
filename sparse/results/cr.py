#!/usr/bin/python3
import sys
import numpy as np
import matplotlib.pyplot as plt
import string
import math

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
                flops += 2 * int(data[i+2].split()[0].translate(t)) # r5301c7
                flops += 4 * int(data[i+4].split()[0].translate(t)) # r5310c7
                flops = flops/1000000 # Change flops to Mflops
                res_x += [flops/1663000] # Total data is 16630MB
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
        print('Threads, MFlops,\tOperation Intensity')
        for i in range(len(all_res_x[j])):
            print(threads[i],'\t', all_res_y[j][i], all_res_x[j][i])
        ax.plot(all_res_x[j],all_res_y[j],label=labels[j], marker=markers[j])
        j += 1
    ax.set_xlim(0.125,2)
    peak_band_x, peak_band_y = abline(135106.598, 0)
    print(peak_band_x, peak_band_y)
    ax.plot(peak_band_x, peak_band_y, label="Peak Bandwidth")
    #ax.plot(peak_band_x, [33782000000 for i in peak_band_x], label="Maximum Flop/s")
    # Max flop/s is 168910 MFlop/s
    ax.plot(ax.get_xlim(), [168910 for i in ax.get_xlim()], label="Maximum MFlop/s (32 cores)")
    ax.set(xlabel='Operational Intensity (MFlops/MByte)', ylabel='Attainable MFlop/s (32 cores)')
    ax.set_yscale('log', basey=10)
    ax.set_xscale('log', basex=10)
    #ax.set_xticklabels([0,1,2,4,8,16])
    ax.set_yticklabels([math.pow(2, i) for i in range(7,18)])
    ax.legend()
    plt.tight_layout()
    fig.savefig('roofline2.png')
