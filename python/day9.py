import math
import re
import collections
import queue
import functools
import sys
import numpy as np


sys.setrecursionlimit(10000000)
np.set_printoptions(threshold=sys.maxsize)


def ints(line: str):
    return [int(x) for x in re.findall("-?\d+", line)]


def parse(filename):
    lines = open(filename).readlines()
    data = [ints(line) for line in lines]
    return data


def solve_lst(lst):
    if all(x == 0 for x in lst):
        return 0

    nxt = [y - x for x, y in zip(lst, lst[1:])]
    return solve_lst(nxt) + lst[-1]


def solve_a(data):
    return sum([solve_lst(lst) for lst in data])


def solve_b(data):
    return sum([solve_lst(lst[::-1]) for lst in data])


if __name__ == "__main__":
    data = parse("day9_test.in")
    print("Test:")
    sol = solve_a(data)
    print(f"A: {sol=}")
    sol = solve_b(data)
    print(f"B: {sol=}")

    print("-----------")
    data = parse("day9.in")
    print("Full:")
    sol = solve_a(data)
    print(f"A: {sol=}")
    sol = solve_b(data)
    print(f"B: {sol=}")
