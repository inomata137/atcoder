from math import comb
import itertools as it

A = list(map(int, input().split()))

def score(counts: list[int]):
    assert len(counts) == 6
    assert sum(counts) == 5
    nums: list[int] = []
    for i in range(6):
        nums += [A[i]] * counts[i]
    return max(num * nums.count(num) for num in nums)

def merge_counts(x, y):
    assert len(x) == 6
    assert len(y) == 6
    return [x[i] + y[i] for i in range(6)]

def count(nums: tuple[int, ...]):
    assert len(nums) <= 5
    c = [0] * 6
    for num in nums:
        c[num] += 1
    return c

def prob(counts: list[int]) -> float:
    assert len(counts) == 6
    d = sum(counts)
    p: float = 1 / (6 ** d)
    for c in counts:
        p *= comb(d, c)
        d -= c
    return p

def key(k: int, s: list[int]):
    return f"{k}-{s}"

cache = {}
def dp(k: int, s: list[int]):
    if key(k, s) in cache:
        return cache[key(k, s)]
    psize = 5 - sum(s)
    res = 0.
    for p in it.combinations_with_replacement([0, 1, 2, 3, 4, 5], psize):
        pcounts = count(p)
        pprob= prob(pcounts)
        if k == 1:
            res += pprob * score(merge_counts(s, pcounts))
        else:
            res += pprob * max(dp(k-1, merge_counts(s, t)) for t in it.product(*[range(pn + 1) for pn in pcounts]))

    cache[key(k, s)] = res

    return res

print(dp(3, [0] * 6))
