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
    label1 = (sys.argv[1].split('_')[0])
    label2 = (sys.argv[2].split('_')[0])
    res1 = parse_file(sys.argv[1])
    res2 = parse_file(sys.argv[2])
    print(res1[0][0])
    for res in res1:
        get_Gb(res)
    for res in res2:
        get_Gb(res)
    print(res1[0][0])


    functions = ['Copy', 'Mul', 'Add', 'Triad', 'Dot']
    x_axis = [1,2] + list(range(4,37,4))
    i = 0
    while i < len(functions):
        print('Graphing', functions[i])
        fig, ax = plt.subplots()
        ax.plot(x_axis, res1[i], label=label1, marker='.')
        ax.plot( x_axis, res2[i], label=label2, marker='.')
        ax.set(xlabel='Number of threads', ylabel='Bandwidth GBytes/sec',
               title=functions[i])
        ax.set_xticklabels(x_axis)
        ax.legend()
        fig.savefig(functions[i] + '.png')
        i += 1
