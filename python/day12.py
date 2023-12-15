from tqdm import tqdm

import numpy as np
from collections import defaultdict, Counter, deque
from math import gcd, prod
import re
import sys

sys.setrecursionlimit(100000)


def solve(data):
    # dp[i][r1][r2][r3] = max(dp[i + 1][r1][r2][r3], dp[i][r1 - 1][r2][r3] + 1) 
    s, nums = data
    dp = np.zeros((len(s), *nums))
    for i in range(len(s) - 1, -1, -1):
        for k in range(len(nums) - 1, -1, -1):
            for x in range(nums[k] + 1):
                idx = nums[:k] + [x] 
                while len(idx) < len(nums):
                    idx.append(0)
                dp[i][*idx] = dp[i + 1][*idx]


    idx = [0 for _ in range(len(dp.shape))]
    return dp[*idx]


lines = [line.strip() for line in open("day12.in").readlines()]
data = []
for line in lines:
    parts = line.split()
    data.append([parts[0].strip(), [int(x) for x in parts[1].split(',')]])

print(data)
solve(data[0])
