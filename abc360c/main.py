n = int(input())
a = list(map(int, input().split()))
w = list(map(int, input().split()))

work = [[] for _ in range(n)]

for i in range(n):
    work[a[i] - 1].append(w[i])

ans = 0

for i in range(n):
    work[i].sort()
    for j in range(len(work[i]) - 1):
        ans += work[i][j]

print(ans)
