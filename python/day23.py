from collections import defaultdict, deque
import sys

sys.setrecursionlimit(1_000_000)


grid = [[c for c in line.strip()] for line in open("input/day23.in").readlines()]


def make_graph(grid):
    edges = defaultdict(set)
    for r, row in enumerate(grid):
        for c, ch in enumerate(row):
            if ch == '.':
                for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                    if not (0 <= r + dr < len(grid) and 0 <= c + dc < len(row)):
                        continue
                    if grid[r + dr][c + dc] == '.':
                        edges[(r, c)].add(((r + dr, c + dc), 1))
                        edges[(r + dr, c + dc)].add(((r, c), 1))
            elif ch == '>':
                edges[(r, c)].add(((r, c + 1), 1))
                edges[(r, c - 1)].add(((r, c), 1))
            elif ch == 'v':
                edges[(r, c)].add(((r + 1, c), 1))
                edges[(r - 1, c)].add(((r, c), 1))

    return edges


def compress_graph(graph):
    while True:
        for node, neighbors in graph.items():
            if len(neighbors) == 2:
                del graph[node]
                (n1, d1), (n2, d2) = neighbors
                graph[n1].remove((node, d1))
                graph[n2].remove((node, d2))
                graph[n1].add((n2, d1 + d2))
                graph[n2].add((n1, d1 + d2))
                break
        else:
            break

    return graph


def dfs(graph, src, dst):
    queue = deque([(src, 0)])
    visited = set()
    best = 0

    while queue:
        node, d = queue.pop()
        if d == -1:
            visited.remove(node)
            continue
        if node in visited:
            continue

        if node == dst:
            best = max(best, d)

        visited.add(node)
        queue.append((node, -1))
        for (nxt, dist) in graph[node]:
            queue.append((nxt, d + dist))

    return best


graph = make_graph(grid)
R, C = len(grid), len(grid[0])
print(dfs(graph, (0, 1), (R - 1, C - 2)))

for r, row in enumerate(grid):
    for c, ch in enumerate(row):
        if ch == '>' or ch == 'v':
            grid[r][c] = '.'

graph = make_graph(grid)
graph = compress_graph(graph)
print(dfs(graph, (0, 1), (R - 1, C - 2)))
