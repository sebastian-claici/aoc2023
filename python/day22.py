import re
from collections import defaultdict


def ints(s):
    return list(map(int, re.findall(r"\d+", s)))


bricks = [
    tuple(ints(x)) for x in open("input/day22.in").read().strip().split("\n")
]


def down(brick):
    return (
        brick[0],
        brick[1],
        brick[2] - 1,
        brick[3],
        brick[4],
        brick[5] - 1,
    )


def positions(brick):
    for x in range(brick[0], brick[3] + 1):
        for y in range(brick[1], brick[4] + 1):
            for z in range(brick[2], brick[5] + 1):
                yield (x, y, z)


def disintegrate(brick):
    falling = set()

    def falls(brick):
        if brick in falling:
            return
        falling.add(brick)
        for parent in above[brick]:
            if below[parent] & falling == below[parent]:
                falls(parent)

    falls(brick)
    return len(falling)


settled = {}
fallen = []
for brick in sorted(bricks, key=lambda brick: brick[2]):
    while brick[2] > 0 and all(pos not in settled for pos in positions(down(brick))):
        brick = down(brick)
    for pos in positions(brick):
        settled[pos] = brick
    fallen.append(brick)

above = defaultdict(set)
below = defaultdict(set)
for brick in fallen:
    contained = set(positions(brick))
    for pos in positions(down(brick)):
        if pos in settled and pos not in contained:
            above[settled[pos]].add(brick)
            below[brick].add(settled[pos])


p1 = 0
p2 = 0
for brick in fallen:
    wouldfall = disintegrate(brick)
    p1 += wouldfall == 1
    p2 += wouldfall - 1

print(p1, p2)
