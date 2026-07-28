#!/usr/bin/env python3

import argparse
import csv
import sys

try:
    import matplotlib.pyplot as plt
except ModuleNotFoundError as error:
    sys.exit(
        f"{error.name} is required: install it with `python3 -m pip install matplotlib`"
    )


def parse_args():
    parser = argparse.ArgumentParser(
        description="Visualize a sampled signal and its one-sided power spectrum."
    )
    parser.add_argument("signal", help="CSV file produced by --signal-output")
    parser.add_argument("spectrum", help="CSV file produced by --spectrum-output")
    parser.add_argument(
        "--output",
        metavar="FILE",
        help="write a figure instead of opening an interactive window",
    )
    return parser.parse_args()


def read_series(filename, index_column, value_column):
    indices = []
    values = []

    with open(filename, newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        required_columns = {index_column, value_column}
        if reader.fieldnames is None or not required_columns.issubset(
            reader.fieldnames
        ):
            expected = f"{index_column},{value_column}"
            raise ValueError(f"{filename}: expected CSV columns {expected}")

        for line_number, row in enumerate(reader, start=2):
            try:
                indices.append(int(row[index_column]))
                values.append(float(row[value_column]))
            except (TypeError, ValueError) as error:
                raise ValueError(
                    f"{filename}:{line_number}: invalid numeric value"
                ) from error

    if not indices:
        raise ValueError(f"{filename}: no data rows found")

    return indices, values


def create_figure(samples, signal, frequency_bins, power):
    figure, axes = plt.subplots(
        nrows=1,
        ncols=2,
        figsize=(12, 4.5),
        constrained_layout=True,
    )

    axes[0].plot(samples, signal)
    axes[0].set_title("Signal")
    axes[0].set_xlabel("sample")
    axes[0].set_ylabel("value")
    axes[0].grid(alpha=0.25)

    axes[1].plot(frequency_bins, power, marker="o", markersize=4)
    axes[1].set_title("One-sided power spectrum")
    axes[1].set_xlabel("frequency bin")
    axes[1].set_ylabel("mean-square power")
    axes[1].set_ylim(bottom=0.0)
    axes[1].grid(alpha=0.25)

    figure.suptitle("FFTW signal and power spectrum")
    return figure


def main():
    args = parse_args()
    try:
        samples, signal = read_series(args.signal, "sample", "value")
        frequency_bins, power = read_series(args.spectrum, "frequency_bin", "power")
    except (OSError, ValueError) as error:
        sys.exit(error)

    figure = create_figure(samples, signal, frequency_bins, power)
    if args.output is None:
        plt.show()
    else:
        figure.savefig(args.output, dpi=150)
        print(f"Figure written to {args.output}")


if __name__ == "__main__":
    main()
