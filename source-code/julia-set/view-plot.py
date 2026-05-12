#!/usr/bin/env python3

import argparse
import matplotlib.pyplot as plt
import numpy as np

def parse_args():
    parser = argparse.ArgumentParser(description='Plot view data from a file.')
    parser.add_argument('filename', help='Path to the input file containing view data')
    return parser.parse_args()

def main():
    args = parse_args()
    
    # Load data from the file
    data = np.loadtxt(args.filename)

    plt.figure(figsize=(10, 6))
    plt.imshow(data, cmap='viridis', aspect='equal')
    plt.show()


if __name__ == '__main__':
    main()
