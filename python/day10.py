from tqdm import tqdm

import numpy as np
from collections import defaultdict, Counter, deque
from math import gcd, prod
import re
import sys

sys.setrecursionlimit(100000)


def bfs(arr, node):
    queue = deque([node])
    dists = {node: 0}
    visited = set([node])
    while queue:
        nxt = deque.popleft(queue)

        for neigh in neighbors(arr, nxt):
            if neigh in visited:
                continue
            visited.add(neigh)
            dists[neigh] = dists[nxt] + 1
            queue.append(neigh)

    return dists, visited


def helper(arr, node, moves):
    r, c = node
    m, n = len(arr), len(arr[r])

    result = []
    for dr, dc in moves:
        if 0 <= r + dr < m and 0 <= c + dc < n:
            result.append((r + dr, c + dc))
    return result


def neighbors(arr, node):
    r, c = node
    if arr[r][c] == '|':
        return helper(arr, node, [(1, 0), (-1, 0)])
    elif arr[r][c] == '-':
        return helper(arr, node, [(0, -1), (0, 1)])
    elif arr[r][c] == 'L':
        return helper(arr, node, [(-1, 0), (0, 1)])
    elif arr[r][c] == 'J':
        return helper(arr, node, [(-1, 0), (0, -1)])
    elif arr[r][c] == '7':
        return helper(arr, node, [(1, 0), (0, -1)])
    elif arr[r][c] == 'F':
        return helper(arr, node, [(1, 0), (0, 1)])


lines = [line.strip() for line in open("day10.in").readlines()]
arr = [[c for c in line.strip()] for line in lines]

pos = None
m, n = len(arr), len(arr[0])
moves = [(1, 0), (-1, 0), (0, 1), (0, -1)]
for r in range(m):
    for c in range(n):
        if arr[r][c] == 'S':
            pos = (r, c)
            break
    if pos is not None:
        break


arr[pos[0]][pos[1]] = 'J'
dists, on_loop = bfs(arr, pos)
print("Part A:", max(dists.values()))

ans = 0
for r in range(m):
    crosses = 0
    corner = None
    for c in range(n):
        if (r, c) in on_loop:
            pipe = arr[r][c]
            if pipe == '-':
                continue
            if pipe == '|':
                crosses += 1
            elif corner:
                if corner == 'L' and pipe == '7':
                    crosses += 1
                elif corner == 'F' and pipe == 'J':
                    crosses += 1
                corner = None
            else:
                corner = pipe
        elif crosses % 2 == 1:
            ans += 1

print("Part B:", ans)
