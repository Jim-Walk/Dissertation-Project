#!/usr/bin/python3
import sys
import matplotlib.pyplot as plt

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res = []
    i = 0
    while i < len(data):
        if data[i].split()[0] == 'Rate':
            res += [float(data[i].split()[-1])]
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

    #for res in all_results:
    #    get_speedup(res)

    x_axis = [1,2] + list(range(4,37,4))
    j = 0
    fig, ax = plt.subplots()
    while j < len(all_results):
        ax.plot(x_axis, all_results[j], label=labels[j], marker='.')
        j += 1
    t = 'Sparse Matrix Multiplication'
    ax.set(xlabel='Number of threads', ylabel='Time (seconds)',title=t)
    ax.set_xticks(x_axis)
    ax.set_xticklabels(x_axis)
    ax.legend()
    fig.savefig('time.png')
