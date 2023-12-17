import numpy as np
import heapq


ROT_R = np.array([[0, 1], [-1, 0]])
ROT_L = np.array([[0, -1], [1, 0]])


def dijkstra(costs, start, part2):
    R, C = len(costs), len(costs[0])
    r, c = start
    queue = [(0, (r, c, 0, 1, 0)), (0, (r, c, 1, 0, 0))]
    heapq.heapify(queue)
    dists = {(r, c, 0, 1, 0): 0, (r, c, 1, 0, 0): 0}

    while queue:
        cst, (r, c, dr, dc, num) = heapq.heappop(queue)

        moves = [[dr, dc], ROT_R @ [dr, dc], ROT_L @ [dr, dc]]
        for (ndr, ndc) in moves:
            cnt = num + 1 if (dr, dc) == (ndr, ndc) else 1
            if part2:
                is_good = (cnt <= 10) and ((dr, dc) == (ndr, ndc) or num >= 4)
            else:
                is_good = cnt <= 3

            if not (0 <= r + ndr < R and 0 <= c + ndc < C and is_good):
                continue

            state = (r + ndr, c + ndc, ndr, ndc, cnt)
            curr = cst + costs[r + ndr][c + ndc]
            if curr < dists.get(state, float("inf")):
                dists[state] = curr
                heapq.heappush(queue, (curr, state))

    return dists


costs = [[int(x) for x in line.strip()] for line in open("input/day17.in").readlines()]

dists = dijkstra(costs, (0, 0), False)
last = (len(costs) - 1, len(costs[0]) - 1)
ans = float("inf")
for (r, c, _, _, _), v in dists.items():
    if r == last[0] and c == last[1] and v <= ans:
        ans = v
print(ans)

dists = dijkstra(costs, (0, 0), True)
last = (len(costs) - 1, len(costs[0]) - 1)
ans = float("inf")
for (r, c, _, _, cnt), v in dists.items():
    if r == last[0] and c == last[1] and cnt >= 4 and v <= ans:
        ans = v
print(ans)
