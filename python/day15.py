from collections import defaultdict

data = open("./day15.in").read()

print(data)


def get_hash(line):
    ans = 0
    for c in line:
        ans += ord(c)
        ans *= 17
        ans %= 256

    return ans


result = defaultdict(dict)

for p in data.split(","):
    p = p.strip()
    if '-' in p:
        pat = p.removesuffix('-')
        h = get_hash(pat)
        if pat in result[h]:
            del result[h][pat]
    else:
        pat, val = p.split('=')
        val = int(val)
        h = get_hash(pat)
        result[h][pat] = val

ans = 0
for h in result:
    for i, (pat, val) in enumerate(result[h].items()):
        ans += (h + 1) * (i + 1) * val

print(ans)
