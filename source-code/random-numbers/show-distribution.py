#!/usr/bin/env python3

import matplotlib.pyplot as plt
import sys


def show_distr(distr, title=None):
    plt.hist(distr, bins=20)
    if title is not None:
        plt.title(title)
    plt.xlabel('Value')
    plt.ylabel('Frequency')
    plt.show()


if __name__ == '__main__':
    values = [float(line.strip()) for line in sys.stdin if line.strip()]
    show_distr(values, title='Distribution of Values')
