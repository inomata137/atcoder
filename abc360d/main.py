from collections import deque

n, t = list(map(int, input().split()))
s = [*input()]
x = list(map(int, input().split()))

dir = {}

for i in range(n):
    dir[x[i]] = s[i]

x = sorted(x)

ans = 0

r = deque()

for i in range(n):
    if dir[x[i]] == '0':
        while len(r) > 0 and r[0] < x[i] - 2 * t:
            r.popleft()
        ans += len(r)
    else:
        r.append(x[i])

print(ans)
