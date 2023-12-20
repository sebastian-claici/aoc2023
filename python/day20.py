from collections import deque, defaultdict
from copy import deepcopy
from math import lcm


def push(graph, flips, nands, src, dst, pulse):
    if dst in flips:
        if pulse:
            return []
        flips[dst] = not flips[dst]
        next_pulse = flips[dst]
    elif dst in nands:
        nands[dst][src] = pulse
        next_pulse = not all(nands[dst].values())
    elif dst in graph:
        next_pulse = pulse
    else:
        return []

    return [(dst, nxt, next_pulse) for nxt in graph[dst]]


def run(graph, flips, nands):
    queue = deque([("button", "broadcaster", False)])
    counts = defaultdict(int)

    while queue:
        src, dst, pulse = queue.popleft()
        counts[pulse] += 1
        queue.extend(push(graph, flips, nands, src, dst, pulse))

    return counts


def find_cycles(graph, flips, nands, needed):
    cycles = {}
    for idx in range(1_000_000):
        queue = deque([("button", "broadcaster", False)])

        while queue:
            if len(cycles) == len(needed):
                break

            src, dst, pulse = queue.popleft()
            if not pulse:
                if dst in needed and dst not in cycles:
                    cycles[dst] = idx + 1

            queue.extend(push(graph, flips, nands, src, dst, pulse))

    return cycles


flips = {}
nands = {}
graph = {}

for line in open("input/day20.in").readlines():
    source, dsts = line.split(" -> ")
    dsts = list(map(str.strip, dsts.split(",")))

    if source[0] == "%":
        source = source[1:]
        flips[source] = False
    elif source[0] == "&":
        source = source[1:]
        nands[source] = {}

    graph[source] = dsts

for src, dsts in graph.items():
    for dst in dsts:
        if dst in nands:
            nands[dst][src] = False

flips_f = deepcopy(flips)
nands_f = deepcopy(nands)

counts = defaultdict(int)
for _ in range(1000):
    round = run(graph, flips, nands)
    counts[True] += round[True]
    counts[False] += round[False]

ans = counts[True] * counts[False]
print(ans)

cycles = find_cycles(graph, flips_f, nands_f, ['hh', 'lk', 'fn', 'fh'])
print(lcm(*cycles.values()))
