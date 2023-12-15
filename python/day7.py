def score(hand):
    return sorted(map(hand.count, hand), reverse=True)


def eval(line, face):
    hand, bid = line.split()
    hand = hand.translate(str.maketrans("TJQKA", face))
    rank = max(score(hand.replace("0", r)) for r in hand)
    return rank, hand, int(bid)


for face in "ABCDE", "A0CDE":
    print(
        sum(
            rank * bid
            for rank, (*_, bid) in enumerate(
                sorted(map(lambda line: eval(line, face), open("input.in"))), start=1
            )
        )
    )
