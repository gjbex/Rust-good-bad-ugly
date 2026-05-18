#!/usr/bin/env python3

import sys

try:
    import plotly.graph_objects as go
except ModuleNotFoundError as error:
    sys.exit(f"{error.name} is required: install it with `python3 -m pip install plotly`")


def show_distr(distr, title=None):
    figure = go.Figure(data=[go.Histogram(x=distr, nbinsx=20)])
    if title is not None:
        figure.update_layout(title=title)
    figure.update_layout(xaxis_title="Value", yaxis_title="Frequency")
    figure.show()


if __name__ == '__main__':
    values = [float(line.strip()) for line in sys.stdin if line.strip()]
    show_distr(values, title='Distribution of Values')
