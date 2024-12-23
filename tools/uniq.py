import re

with open("gold01.log") as f:
  lines = f.readlines()

uniq = {}
for line in lines:
  m = re.search(r"> (0x(CB)?[0-9A-F]{2})", line)
  if m:
    uniq[m.group(1)] = True
  else:
    print(line)
print(uniq.keys())
