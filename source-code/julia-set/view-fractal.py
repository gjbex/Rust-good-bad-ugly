#!/usr/bin/env python3

import argparse
import sys

import numpy as np

try:
    import plotly.graph_objects as go
except ModuleNotFoundError as error:
    sys.exit(f"{error.name} is required: install it with `python3 -m pip install plotly`")


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

    figure = go.Figure(
        data=[
            go.Heatmap(
                z=data,
                colorscale="Viridis",
                colorbar={"title": "Iterations"},
            )
        ]
    )
    figure.update_yaxes(scaleanchor="x", scaleratio=1)
    figure.update_layout(xaxis_title="x index", yaxis_title="y index")
    figure.show()


if __name__ == '__main__':
    main()
