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
    elif state == "R":
        continue

    rule = rules[state]
    for cmd in rule:
        if ":" in cmd:
            cond, nxt = cmd.split(":")
            var = cond[0]
            op = cond[1]
            x = int(cond[2:])

            new_ranges = {k: v for k, v in ranges.items() if k != var}
            low, hi = ranges[var]
            if op == '<':
                queue.append((nxt, {var: (low, x), **new_ranges}))
                ranges = {var: (x, hi), **new_ranges}
            else:
                queue.append((nxt, {var: (x + 1, hi), **new_ranges}))
                ranges = {var: (low, x + 1), **new_ranges}
        else:
            queue.append((cmd, ranges))
            break
print(ans)
