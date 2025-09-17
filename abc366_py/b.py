n = int(input())

lines: list[str] = []
width = 0
for i in range(n):
    line = input()
    width = max(width, len(line))
    lines.append(line)

tlist: list[str] = ['' for _ in range(width)]

for s in lines:
    s2 = s + '*' * (width - len(s))
    for i in range(len(s2)):
        if tlist[i] == '' and s2[i] == '*':
            continue
        tlist[i] = s2[i] + tlist[i]

for t in tlist:
    print(t)
