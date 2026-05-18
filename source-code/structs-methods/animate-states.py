#!/usr/bin/env python3

import argparse
import sys

try:
    import pandas as pd
    import plotly.graph_objects as go
except ModuleNotFoundError as error:
    sys.exit(
        f"{error.name} is required: install dependencies with "
        "`python3 -m pip install pandas plotly`"
    )


def parse_args():
    parser = argparse.ArgumentParser(
        description="Animate n-body particle states from a CSV file."
    )
    parser.add_argument("filename", help="CSV file produced by --save-states")
    parser.add_argument(
        "--output",
        help="Optional HTML file to write instead of opening an interactive window",
    )
    parser.add_argument(
        "--min-marker-size",
        type=float,
        default=4.0,
        help="Marker diameter for the smallest mass",
    )
    parser.add_argument(
        "--max-marker-size",
        type=float,
        default=18.0,
        help="Marker diameter for the largest mass",
    )
    return parser.parse_args()


def marker_sizes(masses, min_size, max_size):
    min_mass = masses.min()
    max_mass = masses.max()
    if min_mass == max_mass:
        return [0.5 * (min_size + max_size)] * len(masses)
    return min_size + (masses - min_mass) * (max_size - min_size) / (max_mass - min_mass)


def make_trace(frame_data, all_masses, args):
    sizes = marker_sizes(frame_data["mass"], args.min_marker_size, args.max_marker_size)
    return go.Scatter3d(
        x=frame_data["x"],
        y=frame_data["y"],
        z=frame_data["z"],
        mode="markers",
        marker={
            "size": sizes,
            "color": frame_data["mass"],
            "cmin": all_masses.min(),
            "cmax": all_masses.max(),
            "colorscale": "Viridis",
            "colorbar": {"title": "mass"},
            "opacity": 0.85,
        },
        text=[
            f"particle={particle}<br>mass={mass:.6g}"
            for particle, mass in zip(frame_data["particle"], frame_data["mass"])
        ],
        hovertemplate=(
            "%{text}<br>"
            "x=%{x:.6g}<br>"
            "y=%{y:.6g}<br>"
            "z=%{z:.6g}<extra></extra>"
        ),
    )


def axis_range(values):
    value_min = values.min()
    value_max = values.max()
    padding = 0.05 * max(value_max - value_min, 1.0)
    return [value_min - padding, value_max + padding]


def make_figure(data, args):
    steps = sorted(data["step"].unique())
    if not steps:
        raise ValueError(f"no particle states found in {args.filename}")

    all_masses = data["mass"]
    first_frame = data[data["step"] == steps[0]]
    frames = [
        go.Frame(
            data=[make_trace(data[data["step"] == step], all_masses, args)],
            name=str(step),
        )
        for step in steps
    ]

    figure = go.Figure(
        data=[make_trace(first_frame, all_masses, args)],
        frames=frames,
        layout=go.Layout(
            scene={
                "xaxis": {"title": "x", "range": axis_range(data["x"])},
                "yaxis": {"title": "y", "range": axis_range(data["y"])},
                "zaxis": {"title": "z", "range": axis_range(data["z"])},
                "aspectmode": "cube",
            },
            updatemenus=[
                {
                    "type": "buttons",
                    "showactive": False,
                    "buttons": [
                        {
                            "label": "Play",
                            "method": "animate",
                            "args": [
                                None,
                                {
                                    "frame": {"duration": 80, "redraw": True},
                                    "fromcurrent": True,
                                },
                            ],
                        },
                        {
                            "label": "Pause",
                            "method": "animate",
                            "args": [
                                [None],
                                {
                                    "frame": {"duration": 0, "redraw": False},
                                    "mode": "immediate",
                                },
                            ],
                        },
                    ],
                }
            ],
            sliders=[
                {
                    "steps": [
                        {
                            "label": str(step),
                            "method": "animate",
                            "args": [
                                [str(step)],
                                {
                                    "frame": {"duration": 0, "redraw": True},
                                    "mode": "immediate",
                                },
                            ],
                        }
                        for step in steps
                    ],
                    "currentvalue": {"prefix": "step "},
                }
            ],
        ),
    )
    return figure


def main():
    args = parse_args()
    data = pd.read_csv(args.filename)
    figure = make_figure(data, args)
    if args.output:
        figure.write_html(args.output)
    else:
        figure.show()


if __name__ == "__main__":
    main()
