#!/usr/bin/env python3

import argparse
import csv
import math
import sys

try:
    import matplotlib.pyplot as plt
except ModuleNotFoundError:
    sys.exit("matplotlib is required: install it with `python3 -m pip install matplotlib`")


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

    _, axes = plt.subplots(nrows=1, ncols=2, figsize=(12, 5), constrained_layout=True)

    axes[0].plot(steps, potential_energy, label="potential")
    axes[0].plot(steps, kinetic_energy, label="kinetic")
    axes[0].plot(steps, total_energy, label="total")
    axes[0].set_xlabel("step")
    axes[0].set_ylabel("energy")
    axes[0].legend()
    axes[0].grid(True, alpha=0.3)

    axes[1].plot(steps, distances)
    axes[1].set_xlabel("step")
    axes[1].set_ylabel("center-of-mass displacement")
    axes[1].grid(True, alpha=0.3)

    plt.show()


if __name__ == "__main__":
    main()
