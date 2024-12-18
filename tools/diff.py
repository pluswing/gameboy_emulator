import re
import sys

a = sys.argv[1]
b = sys.argv[2]

with open(a) as f:
  alines = f.readlines()

with open(b) as f:
  blines = f.readlines()

for i, (a, b) in enumerate(zip(alines, blines)):
  ainst = re.sub(r"AF.*", "", a)
  binst = re.sub(r"AF.*", "", b)
  if ainst != binst:
    print(f"{i + 1}\na:{a}\nb:{b}")
    break
