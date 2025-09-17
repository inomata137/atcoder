from collections import deque

Rt, Ct, Ra, Ca = map(int, input().split())
N, M, L = map(int, input().split())
S = deque()
for _ in range(M):
    s, a = input().split()
    S.append((s, int(a)))
T = deque()
for _ in range(L):
    t, b = input().split()
    T.append((t, int(b)))

U = []
while len(S) > 0 and len(T) > 0:
    s, a = S.popleft()
    t, b = T.popleft()
    m = min(a, b)
    U.append((s, t, m))
    if a > m:
        S.appendleft((s, a - m))
    if b > m:
        T.appendleft((t, b - m))

def move(s, t):
    dr = 0
    dc = 0
    match s:
        case "U":
            dr -= 1
        case "D":
            dr += 1
        case "L":
            dc -= 1
        case "R":
            dc += 1
    match t:
        case "U":
            dr += 1
        case "D":
            dr -= 1
        case "L":
            dc += 1
        case "R":
            dc -= 1
    return dr, dc

count = 0
Dr = Rt - Ra
Dc = Ct - Ca
for s, t, n in U:
    dr, dc = move(s, t)
    if dr == 0 and dc == 0:
        if Dr == 0 and Dc == 0:
            count += n
    elif dr == 0:
        m = -Dc // dc
        if m > 0 and m <= n and Dc % dc == 0 and Dr == 0:
            count += 1
        Dc += dc * n
    elif dc == 0:
        m = -Dr // dr
        if m > 0 and m <= n and Dr % dr == 0 and Dc == 0:
            count += 1
        Dr += dr * n
    else:
        m1 = -Dc // dc
        m2 = -Dr // dr
        if m1 == m2 and m1 > 0 and m1 <= n and Dr % dr == 0 and Dc % dc == 0:
            count += 1
        Dc += dc * n
        Dr += dr * n

print(count)
