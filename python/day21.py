import numpy as np
from numpy.linalg import matrix_power
from scipy.sparse import dok_matrix

from collections import deque, defaultdict
from functools import lru_cache



grid = [[c for c in line.strip()] for line in open("input/day21.in").readlines()]

start = (0, 0)
for r, line in enumerate(grid):
    for c, s in enumerate(line):
        if s == 'S':
            start = (r, c)
            break

R, C = len(grid), len(grid[0])
matrix = dok_matrix((R * C, R * C))
adjacency = defaultdict(list)
for r, line in enumerate(grid):
    for c, s in enumerate(line):
        moves = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        for dr, dc in moves:
            nr, nc = (r + dr), (c + dc)
            if 0 <= nr < R and 0 <= nc < C and grid[nr][nc] != '#':
                adjacency[(r, c)].append((nr, nc))
                matrix[c * R + r, nc * R + nr] = 1

s_vec = np.zeros(R * C)
s_vec[start[1] * R + start[0]] = 1
print(np.count_nonzero((matrix ** 26501365).T @ s_vec))
