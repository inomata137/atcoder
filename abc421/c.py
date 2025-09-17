n = int(input())
s = input()

src_a_pos = [i for i in range(2*n) if s[i] == 'A']

target_a_pos = [2*k for k in range(n)]
ans1 = sum([abs(target_a_pos[i] - src_a_pos[i]) for i in range(n)])

target_a_pos = [2*k + 1 for k in range(n)]
ans2 = sum([abs(target_a_pos[i] - src_a_pos[i]) for i in range(n)])

print(min(ans1, ans2))
