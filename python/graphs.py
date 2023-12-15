from collections import deque
import queue


def dijkstra(arr, node):
    moves = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    dist = {node: 0}
    best = queue.PriorityQueue()
    best.put((0, node))

    while not best.empty():
        d, node = best.get()
        for x, y in neighbors(arr, node, moves):
            cost = arr[x][y]
            if cost + d < dist.get((x, y), float('inf')):
                dist[(x, y)] = cost + d
                best.put((cost + d, (x, y)))

    return dist


def bfs(arr, node):
    moves = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    queue = deque([node])
    dists = {node: 0}
    visited = set([node])
    while queue:
        nxt = deque.popleft(queue)
        queue = queue[1:]

        for neigh in neighbors(arr, nxt, moves):
            if neigh in visited:
                continue
            visited.add(neigh)
            dists[neigh] = dists[nxt] + 1
            queue.append(neigh)

    return dists, visited


def dfs(arr, node, seen):
    moves = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    for neigh in neighbors(arr, node, moves):
        if neigh not in seen:
            seen.add(neigh)
            dfs(arr, neigh, seen)


def neighbors(arr, node, moves):
    r, c = node
    m, n = len(arr), len(arr[r])

    result = []
    for dr, dc in moves:
        if 0 <= r + dr < m and 0 <= c + dc < n:
            result.append((r + dr, c + dc))
    return result

