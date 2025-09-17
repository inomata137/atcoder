H, W = map(int, input().split())
S = []
for _ in range(H):
    S.append(input())

def isblack(x, y):
    return S[y][x] == '#'

for y in range(H):
    for x in range(W):
        if S[y][x] == '.':
            continue
        count = 0
        if x > 0 and isblack(x - 1, y):
            count += 1
        if y > 0 and isblack(x, y - 1):
            count += 1
        if x < W-1 and isblack(x + 1, y):
            count += 1
        if y < H-1 and isblack(x, y + 1):
            count += 1
        if count in [0, 1, 3]:
            print("No")
            exit()

print("Yes")
