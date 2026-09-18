"""Generate benchmark_parse.svg. Requires matplotlib: pip install matplotlib."""

from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FixedLocator, FuncFormatter


# Criterion central estimates from the README, retaining their displayed units.
SIZES = (10, 100, 1_000, 2_000, 5_000, 10_000)
BENCHMARKS = (
    (
        "parse",
        ("20.415 µs", "107.14 µs", "1.1002 ms", "2.5137 ms", "9.1901 ms", "32.179 ms"),
        ("1.1200 µs", "5.6657 µs", "51.615 µs", "104.65 µs", "275.39 µs", "541.03 µs"),
    ),
    (
        "deparse",
        ("11.715 µs", "66.007 µs", "613.31 µs", "1.2209 ms", "3.0952 ms", "6.3492 ms"),
        ("777.91 ns", "3.6580 µs", "35.260 µs", "70.296 µs", "178.90 µs", "355.46 µs"),
    ),
)


def create_figure():
    plt.rcParams.update({
        "font.family": "DejaVu Sans",
        "font.size": 16.5,
        "text.color": "#24352f",
        "axes.labelcolor": "#43554d",
        "xtick.color": "#43554d",
        "ytick.color": "#43554d",
        "axes.spines.top": False,
        "axes.spines.right": False,
        "axes.spines.left": False,
        "axes.spines.bottom": False,
        "svg.fonttype": "none",
        "svg.hashsalt": "pg-raw-parse-benchmark",
    })
    fig, axes = plt.subplots(1, 2, figsize=(20, 6))
    fig.subplots_adjust(left=0.09, right=0.98, top=0.90, bottom=0.20, wspace=0.28)
    for times, (operation, query_labels, raw_labels) in zip(axes, BENCHMARKS):
        draw_chart(times, operation, query_labels, raw_labels)
    return fig


def draw_chart(times, operation, query_labels, raw_labels):
    x = list(range(len(SIZES)))
    for labels, color, marker, linestyle, name, offset in (
        (raw_labels, "#355ba9", "o", "-", "pg_raw_parse", -30),
        (query_labels, "#b96a47", "s", "--", "pg_query.rs", 20),
    ):
        values = []
        for label in labels:
            value, unit = label.split()
            values.append(float(value) * {"ns": 0.001, "µs": 1, "ms": 1_000}[unit])
        times.plot(x, values, label=name, color=color, marker=marker,
                   linestyle=linestyle, linewidth=2.4, markersize=6)
        for position, value, label in zip(x, values, labels):
            times.annotate(label, (position, value), xytext=(0, offset),
                           textcoords="offset points", ha="center", fontsize=13.5,
                           color=color)

    times.set_yscale("log")
    times.set_ylim(0.15, 100_000)
    times.set_xlim(-0.45, 5.45)
    times.yaxis.set_major_locator(FixedLocator([1, 10, 100, 1_000, 10_000, 100_000]))
    times.yaxis.set_major_formatter(FuncFormatter(
        lambda value, _: f"{value / 1_000:g} ms" if value >= 1_000 else f"{value:g} µs"
    ))
    times.minorticks_off()
    times.set_ylabel(f"Time per {operation}", labelpad=14)
    times.grid(axis="y", color="#e2e8e3", linewidth=0.8)
    times.set_axisbelow(True)
    times.tick_params(axis="both", length=0, pad=12)
    times.legend(loc="lower center", bbox_to_anchor=(0.5, 1.02), ncol=2,
                 frameon=False, borderaxespad=0)
    times.text(1, 1.05, "Log scale", transform=times.transAxes,
               ha="right", fontsize=15, color="#63746b")
    times.set_xticks(x, [f"{size:,}" for size in SIZES])
    times.set_xlabel("Query length (nodes)", labelpad=12)


if __name__ == "__main__":
    output = Path(__file__).resolve().parents[1] / "benchmark_parse.svg"
    figure = create_figure()
    figure.savefig(output, format="svg", metadata={
        "Date": None,
        "Title": "PostgreSQL parse and deparse performance",
        "Description": "Parse and deparse timings for pg_raw_parse and pg_query.rs on Apple M1 Max. Query sizes are equally spaced categories. Both charts use the same logarithmic time scale.",
    })
    plt.close(figure)
    print(output)
