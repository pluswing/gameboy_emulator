import json

with open("instmap.json") as f:
    instmap = json.loads(f.read())

with open("instrs.txt") as f:
    lines = f.readlines()

for line in lines:
    k = line.strip().replace("'", "")
    print(f"{k} => {instmap[k]}")