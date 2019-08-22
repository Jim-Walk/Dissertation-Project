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
                res_x += [flops/1663000] # Total data is 16630MB, 100 times
                flops = flops / float(data[i+7].split()[0])
                res_y += [flops]
        i+=1

    return res_x, res_y

def abline(slope, intercept):
    axes = plt.gca()
    x_vals = np.array(axes.get_xlim())
    y_vals = intercept + slope * x_vals
    return x_vals, y_vals

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


    markers = ["+", "x", "s", "^", "x", "1", "2", "3", "4"]
    threads = [1,2] + list(range(4,37,4))
    j = 0
    fig, ax = plt.subplots()
    while j < len(all_res_x):
        print(labels[j])
        print('Threads, MFlops,\tOperation Intensity')
        for i in range(len(all_res_x[j])):
            print(threads[i],'\t', all_res_y[j][i], all_res_x[j][i])

        ax.plot(all_res_x[j][0:11:2],all_res_y[j][0:11:2],label=labels[j], marker=markers[j],
                linestyle='None') # [0:11:2] gets every other element
        j += 1
    ax.set_xlim(0.1,10)
    peak_band_x, peak_band_y = abline(135106.598, 0)
    peak_band_x1, peak_band_y1 = abline(11563.658, 0)
    ax.plot(peak_band_x, peak_band_y)#, label="Peak bandwidth 32 cores")
    ax.plot(peak_band_x1, peak_band_y1)#, label="Peak bandwidth 1 core")
    # Max flop/s is 168910 MFlop/s
    ax.plot(ax.get_xlim(), [422400 for i in ax.get_xlim()])#, label="Peak MFlop/s 32 cores")
    ax.plot(ax.get_xlim(), [13200 for i in ax.get_xlim()])#, label="Peak MFlop/s 1 core")
    ax.set(xlabel='Operational Intensity (MFlops/MByte)', ylabel='Attained GFlop/s')
    ax.set_yscale('log', basey=10)
    ax.set_xscale('log', basex=10)
    ax.set_xticklabels([0,0.1,1,10])
    #ax.set_xticklabels([0.125,0.13,0.2,0.3,0.4,0.6,1])
    ax.set_yticklabels([int(math.pow(10, i)) for i in range(-1,18)])
    lgd = ax.legend()#bbox_to_anchor=(1.05, 0.6),loc=2, borderaxespad=0.)
    plt.tight_layout()
    fig.savefig('roofline2.png', dpi=200)#, bbox_extra_artists=(lgd,),bbox_inches='tight')
