Q = int(input())

balls: dict[int, int] = {}

for _ in range(Q):
    query = input()
    if query[0] == '1':
        x = int(query.split()[1])
        balls[x] = balls.get(x, 0) + 1
    elif query[0] == '2':
        x = int(query.split()[1])
        balls[x] = balls.get(x, 0) - 1
        if balls[x] == 0:
            del balls[x]
    else:
        print(len(balls))
