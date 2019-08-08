#!/usr/bin/python3
import sys
import numpy as np
import matplotlib.pyplot as plt

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res = []
    i = 0
    while i < len(data):
        if data[i].split()[0] == 'Rate':
            res += [quick_fix(data[i].strip().split(' ')[2])]
        i+=1

    print(res)
    return res

def quick_fix(num_str):
    if num_str[-1] == ',':
        return float(num_str[0:-1:1])
    else:
        return float(num_str)


# Takes a list of times and calculates speedup
# speedup = t1/tn
def get_speedup(res):
    i = 0
    t1 = res[0]
    while i < len(res):
        res[i] = t1/res[i]
        i += 1

def get_Gb(results):
    i = 0
    while i < len(results):
        res[i] = res[i] / 1000
        i += 1

def abline(slope, intercept):
    axes = plt.gca()
    x_vals = np.array(axes.get_xlim())
    y_vals = intercept + slope * x_vals
    return x_vals, y_vals

def flops_per_byte(res):
    flby = []
    for r in res:
        flby += [r/8187]
    return flby

if __name__ == '__main__':
    labels = []
    all_results = []
    i = 1
    while i < len(sys.argv):
        labels += [sys.argv[i].split('.')[0]]
        all_results += [parse_file(sys.argv[i])]
        i += 1

    for res in all_results:
        get_speedup(res)

    x_axis = [1,2] + list(range(4,37,4))
    j = 0
    fig, ax = plt.subplots()
    while j < len(all_results):
        my_y = flops_per_byte(all_results[j])
        ax.plot(my_y, all_results[j], label=labels[j], marker='.')
        j += 1
    peak_band_x, peak_band_y = abline(135106, 0)
    ax.plot(peak_band_x, peak_band_y, label="Peak Bandwidth")
    t = 'Sparse Matrix Multiplication - Roofline'
    ax.set(xlabel='Operational Intensity (MFlops/MByte)', ylabel='Attainable MFlop/s',title=t)
    #ax.set_xticks(x_axis)
    #ax.set_xticklabels(x_axis)
    ax.legend()
    fig.savefig('roofline.png')
