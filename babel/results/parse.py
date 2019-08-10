#!/usr/bin/python3
import sys
import matplotlib.pyplot as plt

def parse_file(f):
    print('Parsing',f)
    with open(f) as data_file:
        data = data_file.readlines()

    res = [[],[],[],[],[]]
    i = 0
    while i < len(data):
        if data[i].split()[0] == 'Copy':
            res[0] += [float(data[i].split()[1])]
            res[1] += [float(data[i+1].split()[1])]
            res[2] += [float(data[i+2].split()[1])]
            res[3] += [float(data[i+3].split()[1])]
            res[4] += [float(data[i+4].split()[1])]
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

    for config_res in all_results:
        for res in config_res:
            get_Gb(res)

    markers = ["o", "v", "s", "^", "x", "1", "2", "3", "4"]
    functions = ['Copy', 'Mul', 'Add', 'Triad', 'Dot']
    x_axis = [1,2] + list(range(4,37,4))
    i = 0
    peak_mem = [153.6 for i in range(len(x_axis))]
    while i < len(functions):
        print('Graphing', functions[i])
        fig, ax = plt.subplots()
        j = 0
        while j < len(all_results):
            ax.plot(x_axis, all_results[j][i], label=labels[j],
                    marker=markers[j])
            j += 1
        ax.plot(x_axis, peak_mem, label='Theoretical Peak', ls='--')
        ax.set(xlabel='Number of threads', ylabel='Memory Bandwidth (MB/s)')
        ax.set_xticks(x_axis)
        ax.set_xticklabels(x_axis)
        lgd = ax.legend(bbox_to_anchor=(1.05, 1),loc=2, borderaxespad=0.)
        fig.savefig(functions[i] + '.png', bbox_extra_artists=(lgd,),
                    bbox_inches='tight')
        i += 1
