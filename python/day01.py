from pathlib import Path

import numpy as np
import pandas as pd


def part_one(inp: pd.DataFrame) -> pd.Series:
    B_sort = inp.B.sort_values().values
    A_sort = inp.A.sort_values().values
    return np.abs(B_sort - A_sort).sum()


def part_two(inp: pd.DataFrame) -> pd.Series:
    A_sort = inp.A.sort_values().values
    B_sort = inp.B.sort_values()
    occ = B_sort.value_counts(sort=False)
    occ = occ[occ.index.isin(A_sort)]
    num_A = np.array([occ[elem] if elem in occ.index else 0 for elem in A_sort])
    return (A_sort * num_A).sum()


if __name__ == "__main__":
    input_path = Path("data/inputs") / "01.txt"
    inp = pd.read_csv(input_path, sep="\s+", header=None, names=["A", "B"])
    result_one = part_one(inp)
    print(result_one)
    result_two = part_two(inp)
    print(result_two)
