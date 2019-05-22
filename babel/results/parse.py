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
            res[0] += [float(data[i].split()[4])]
            res[1] += [float(data[i+1].split()[4])]
            res[2] += [float(data[i+2].split()[4])]
            res[3] += [float(data[i+3].split()[4])]
            res[4] += [float(data[i+4].split()[4])]
        i+=1

    return res

# Takes a list of times and calculates speedup
# speedup = t1/tn
def get_speedup(res):
    i = 1
    while i < len(res):
        res[i] = res[0]/res[i]
        i += 1

if __name__ == '__main__':
    label1 = (sys.argv[1].split('_')[0])
    label2 = (sys.argv[2].split('_')[0])
    res1 = parse_file(sys.argv[1])
    res2 = parse_file(sys.argv[2])
    for res in res1:
        get_speedup(res)
    for res in res2:
        get_speedup(res)


    functions = ['Copy', 'Mul', 'Add', 'Triad', 'Dot']
    x_axis = [1,2] + list(range(4,37,4))
    i = 0
    while i < len(functions):
        print('Graphing', functions[i])
        fig, ax = plt.subplots()
        ax.plot(x_axis, res1[i], label=label1, marker='.')
        ax.plot( x_axis, res2[i], label=label2, marker='.')
        ax.set(xlabel='Number of threads', ylabel='Speedup (t1/tn)',
               title=functions[i])
        ax.legend()
        fig.savefig(functions[i] + '.png')
        i += 1
