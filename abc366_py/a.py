n, t, a = map(int, input().split())

yes = 'Yes' if t > (n // 2) or a > (n // 2) else 'No'
print(yes)
