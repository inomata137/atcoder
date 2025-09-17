def solve():
    na, nb, nc = map(int, input().split())
    """find m that:
    m <= na
    m <= nc
    3m <= na + nb + nc
    """
    m = min(na, nc, (na + nb + nc)//3)
    print(m)

T = int(input())
for _ in range(T):
    solve()
