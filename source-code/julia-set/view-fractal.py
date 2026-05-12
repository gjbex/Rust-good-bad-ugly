#!/usr/bin/env python3

import argparse
import sys

import matplotlib.pyplot as plt
import numpy as np


def parse_args():
    parser = argparse.ArgumentParser(description='Plot Julia set data.')
    parser.add_argument(
        'filename',
        nargs='?',
        default='-',
        help='Path to the input file, or "-" to read from standard input',
    )
    return parser.parse_args()


def load_data(filename):
    if filename == '-':
        return np.loadtxt(sys.stdin)
    return np.loadtxt(filename)


def main():
    args = parse_args()
    data = load_data(args.filename)

    plt.figure(figsize=(10, 6))
    plt.imshow(data, cmap='viridis', aspect='equal')
    plt.colorbar(label='Iterations')
    plt.show()


if __name__ == '__main__':
    main()
