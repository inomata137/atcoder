N, K = map(int, input().split())

L = 1 << N
base = K // L
Kr = K % L

print(0 if Kr == 0 else 1)

for i in range(L):
    b = bin(i)[:1:-1]
    b += "0" * (N - len(b))
    if int(b, 2) < Kr:
        print(f"{base+1}", end="")
    else:
        print(f"{base}", end="")
    if i < L - 1:
        print(" ", end="")

print()
