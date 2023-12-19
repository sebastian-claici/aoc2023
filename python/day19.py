from math import prod
from collections import deque


input = open("input/day19.in").read().strip()

rule_str, parts = input.split("\n\n")
rules = {}
for rule in rule_str.split("\n"):
    state, rest = rule.split("{")
    rules[state] = rest[:-1].split(',')


ans = 0
queue = deque([("in", {"x": (1, 4001), "m": (1, 4001), "a": (1, 4001), "s": (1, 4001)})])
while queue:
    state, ranges = queue.pop()

    if any(lo >= hi for _, (lo, hi) in ranges.items()):
        continue
    if state == "A":
        ans += prod(hi - lo for _, (lo, hi) in ranges.items())
        continue
    if state == "R":
        continue

    for cmd in rules[state]:
        if ":" in cmd:
            cond, nxt = cmd.split(":")
            var, op, x = cond[0], cond[1], int(cond[2:])

            old_ranges = {k: v for k, v in ranges.items() if k != var}
            lo, hi = ranges[var]
            if op == '<':
                queue.append((nxt, {var: (lo, x), **old_ranges}))
                ranges = {var: (x, hi), **old_ranges}
            else:
                queue.append((nxt, {var: (x + 1, hi), **old_ranges}))
                ranges = {var: (lo, x + 1), **old_ranges}
        else:
            queue.append((cmd, ranges))

print(ans)
