s, t = input().split()
l = len(s)
for w in range(1, len(s)):
    for c in range(1, w + 1):
        if s[(c-1)::w] == t:
            print('Yes')
            exit()

print('No')
