#!/usr/bin/python3
import csv
import numpy as np
import matplotlib.pyplot as plt
import scipy.stats as stats
import sys

def scatter(d, comps, scores):
    fig, ax = plt.subplots()
    for xe, ye in d.items():
        xAx = [xe] * len(ye)
        sizes = [ye.count(num)**2.5 * 30 for num in ye]
        ax.scatter(xAx, ye, s=sizes)
    ax.plot(np.unique(comps),
             np.poly1d(np.polyfit(comps, scores,1))(np.unique(comps)))
    ax.set(xlabel="Competency", ylabel="Scores")
    ax.set_yticks([2,3,4,5,6])
    fig.savefig('scatter.png')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Please use one file as the argument for this program")

    else:
        print("Parsing",sys.argv[1])
        d = {}
        scores = []
        comps = []
        with open(sys.argv[1]) as csvfile:
            reader = csv.reader(csvfile)
            next(reader)
            # Dictionary is d[competency] = scores
            for row in reader:
                scores += [int(row[0])]
                comps += [int(row[1])]
                score = [int(row[0])]
                competency = int(row[1])
                if competency in d:
                    d[competency] += score
                else:
                    d[competency] = score
        print("Generating scatter")
        scatter(d, comps, scores)
