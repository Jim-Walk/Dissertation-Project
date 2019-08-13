#!/usr/bin/python3
import csv
import numpy as np
import matplotlib.pyplot as plt
import scipy.stats as stats
import sys

def normal(scores):
    scores.sort()
    s_mean = np.mean(scores)
    s_std = np.std(scores)
    pdf = stats.norm.pdf(scores, s_mean, s_std)
    plt.plot(scores, pdf, '-o')
    plt.hist(scores, normed=True)
    plt.show()

def scatter(scores, competency):
    fig, ax = plt.subplots()
    ax.scatter(competency, scores)
    ax.plot(np.unique(competency),
             np.poly1d(np.polyfit(competency, scores,1))(np.unique(competency)))
    ax.set(xlabel="Competency", ylabel="Scores")
    ax.set_yticks([2,3,4,5,6])
    fig.savefig('scatter.png')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Please use one file as the argument for this program")

    else:
        scores = []; competency = []
        print("Parsing",sys.argv[1])
        with open(sys.argv[1]) as csvfile:
            reader = csv.reader(csvfile)
            next(reader)
            for row in reader:
                scores += [int(row[0])]
                competency += [int(row[1])]
        print(scores)
        print(competency)
        print("Generating normal distribution")
        #normal(scores)
        scatter(scores, competency)
