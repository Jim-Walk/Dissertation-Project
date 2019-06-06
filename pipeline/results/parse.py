#!/usr/bin/python3
import sys
import matplotlib.pyplot as plt

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res = [[],[]] # Mflops, seconds
    i = 0
    while i < len(data):
        if data[i].strip() == 'Solution validates':
            res[0] += [float(data[i+1].split()[2])]
            res[1] += [float(data[i+1].split()[6])]
        i+=1

    return res

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

if __name__ == '__main__':
    labels = []
    all_results = []
    i = 1
    while i < len(sys.argv):
        labels += [sys.argv[i].split('.')[0]]
        all_results += [parse_file(sys.argv[i])]
        i += 1

    for res in all_results:
        get_speedup(res[1])

    measures = ['Rate (Gflops)', 'Speedup']
    x_axis = [1,2] + list(range(4,37,4))
    i = 0
    peak_mem = [153.6 for i in range(len(x_axis))]
    while i < len(measures):
        print('Graphing', measures[i])
        fig, ax = plt.subplots()
        j = 0
        while j < len(all_results):
            ax.plot(x_axis, all_results[j][i], label=labels[j], marker='.')
            j += 1
        ax.set(xlabel='Number of threads', ylabel=measures[i],title='ParRes Pipeline')
        ax.set_xticks(x_axis)
        ax.set_xticklabels(x_axis)
        ax.legend()
        fig.savefig(measures[i] + '.png')
        i += 1
