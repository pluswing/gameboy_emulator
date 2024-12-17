import re

with open("gold01.return_menu.log.s") as f:
  alines = f.readlines()

with open("gold01.shutdown.log.s") as f:
  blines = f.readlines()

for i, (a, b) in enumerate(zip(alines, blines)):
  ainst = re.sub(r"AF.*", "", a)
  binst = re.sub(r"AF.*", "", b)
  if ainst != binst:
    print(f"{i + 1}\na:{a}\nb:{b}")
    break
