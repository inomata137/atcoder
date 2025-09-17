N = int(input())

A: list[list[list[int]]] = [[[0 for z in range(N + 1)] for y in range(N + 1)] for x in range(N + 1)]

for x in range(N):
    for y in range(N):
        Axy = list(map(int, input().split()))
        for z in range(N):
            A[x + 1][y + 1][z + 1] = Axy[z]

for x in range(1, N + 1):
    for y in range(1, N + 1):
        for z in range(1, N + 1):
            A[x][y][z] += A[x-1][y][z]

for x in range(1, N + 1):
    for y in range(1, N + 1):
        for z in range(1, N + 1):
            A[x][y][z] += A[x][y-1][z]

for x in range(1, N + 1):
    for y in range(1, N + 1):
        for z in range(1, N + 1):
            A[x][y][z] += A[x][y][z-1]

Q = int(input())

for _ in range(Q):
    lx, rx, ly, ry, lz, rz = map(int, input().split())
    s1 = A[rx][ry][rz]
    s2 = A[lx - 1][ry][rz] + A[rx][ly - 1][rz] + A[rx][ry][lz - 1]
    s3 = A[lx - 1][ly - 1][rz] + A[lx - 1][ry][lz - 1] + A[rx][ly - 1][lz - 1]
    s4 = A[lx - 1][ly - 1][lz - 1]
    print(s1 - s2 + s3 - s4)
