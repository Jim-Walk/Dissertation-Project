#!/usr/bin/python3

# Generates time graph
import sys
import matplotlib.pyplot as plt

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res = []
    i = 0
    while i < len(data):
        if data[i].split()[0] == 'E-step':
            res += [float(data[i].split()[-1])]
        i+=1

    return res

def get_Gb(results):
    i = 0
    while i < len(results):
        res[i] = res[i] / 1000
        i += 1

if __name__ == '__main__':
    labels = []
    all_results = []
    i = 1
    while i < len(sys.argv):
        labels += [sys.argv[i].split('.')[0]]
        all_results += [parse_file(sys.argv[i])]
        i += 1


    x_axis = [1,2] + list(range(4,37,4))
    j = 0
    fig, ax = plt.subplots()
    while j < len(all_results):
        ax.plot(x_axis, all_results[j], label=labels[j], marker='.')
        j += 1
    t = 'K-Means'
    ax.set(xlabel='Number of threads', ylabel='Time (seconds)',title=t)
    ax.set_xticks(x_axis)
    ax.set_xticklabels(x_axis)
    ax.set_yscale('log', basey=2)
    ax.set_xscale('log', basex=2)
    ax.legend()
    fig.savefig('time.png')
