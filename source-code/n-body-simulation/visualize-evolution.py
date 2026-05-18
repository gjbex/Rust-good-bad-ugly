#!/usr/bin/env python3

import argparse
import csv
import math
import sys

try:
    from plotly.subplots import make_subplots
    import plotly.graph_objects as go
except ModuleNotFoundError as error:
    sys.exit(f"{error.name} is required: install it with `python3 -m pip install plotly`")


def parse_args():
    parser = argparse.ArgumentParser(
        description="Visualize energy and center-of-mass evolution for an n-body run."
    )
    parser.add_argument("filename", help="CSV file produced by --save-evolution")
    return parser.parse_args()


def read_evolution(filename):
    steps = []
    potential_energy = []
    kinetic_energy = []
    total_energy = []
    center_of_mass = []

    with open(filename, newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            steps.append(int(row["step"]))
            potential_energy.append(float(row["potential_energy"]))
            kinetic_energy.append(float(row["kinetic_energy"]))
            total_energy.append(float(row["total_energy"]))
            center_of_mass.append(
                (
                    float(row["center_of_mass_x"]),
                    float(row["center_of_mass_y"]),
                    float(row["center_of_mass_z"]),
                )
            )

    if not steps:
        raise ValueError(f"no evolution records found in {filename}")

    return steps, potential_energy, kinetic_energy, total_energy, center_of_mass


def center_of_mass_distances(center_of_mass):
    x0, y0, z0 = center_of_mass[0]
    distances = []
    for x, y, z in center_of_mass:
        dx = x - x0
        dy = y - y0
        dz = z - z0
        distances.append(math.sqrt(dx * dx + dy * dy + dz * dz))
    return distances


def main():
    args = parse_args()
    steps, potential_energy, kinetic_energy, total_energy, center_of_mass = read_evolution(
        args.filename
    )
    distances = center_of_mass_distances(center_of_mass)

    figure = make_subplots(
        rows=1,
        cols=2,
        subplot_titles=("Energy", "Center-of-mass displacement"),
    )
    figure.add_trace(
        go.Scatter(x=steps, y=potential_energy, mode="lines", name="potential"),
        row=1,
        col=1,
    )
    figure.add_trace(
        go.Scatter(x=steps, y=kinetic_energy, mode="lines", name="kinetic"),
        row=1,
        col=1,
    )
    figure.add_trace(
        go.Scatter(x=steps, y=total_energy, mode="lines", name="total"),
        row=1,
        col=1,
    )
    figure.add_trace(
        go.Scatter(x=steps, y=distances, mode="lines", name="COM displacement"),
        row=1,
        col=2,
    )
    figure.update_xaxes(title_text="step", row=1, col=1)
    figure.update_xaxes(title_text="step", row=1, col=2)
    figure.update_yaxes(title_text="energy", row=1, col=1)
    figure.update_yaxes(title_text="distance", row=1, col=2)
    figure.show()


if __name__ == "__main__":
    main()
