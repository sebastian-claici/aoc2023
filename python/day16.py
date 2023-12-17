from collections import defaultdict, deque


def bfs(grid, start):
    dists = {start: 0}
    rays = set([start])
    queue = deque([start])

    while queue:
        r, c, dr, dc = queue.popleft()
        if grid[r][c] == '\\':
            dr, dc = dc, dr
            moves = [(dr, dc)]
        elif grid[r][c] == '/':
            dr, dc = -dc, -dr
            moves = [(dr, dc)]
        elif abs(dc) == 1 and grid[r][c] == '|':
            moves = [(1, 0), (-1, 0)]
        elif abs(dr) == 1 and grid[r][c] == '-':
            moves = [(0, 1), (0, -1)]
        else:
            moves = [(dr, dc)]

        for dr, dc in moves:
            nr, nc = r + dr, c + dc
            if nr < 0 or nr >= len(grid) or nc < 0 or nc >= len(grid[0]):
                continue
            if (nr, nc, dr, dc) in rays:
                continue
            rays.add((nr, nc, dr, dc))
            queue.append((nr, nc, dr, dc))

    return rays


grid = [line.strip() for line in open("input/day16.in").readlines()]
rays = bfs(grid, (0, 0, 0, 1))

energized = set([(r[0], r[1]) for r in rays])
print(len(energized))

best = 0
R, C = len(grid), len(grid[0])
for r in range(R):
    rays = bfs(grid, (r, 0, 0, 1))
    energized = set([(r[0], r[1]) for r in rays])
    best = max(best, len(energized))

for r in range(R):
    rays = bfs(grid, (r, C - 1, 0, -1))
    energized = set([(r[0], r[1]) for r in rays])
    best = max(best, len(energized))

for c in range(C):
    rays = bfs(grid, (0, c, 1, 0))
    energized = set([(r[0], r[1]) for r in rays])
    best = max(best, len(energized))

for c in range(C):
    rays = bfs(grid, (R - 1, c, -1, 0))
    energized = set([(r[0], r[1]) for r in rays])
    best = max(best, len(energized))

print(best)
