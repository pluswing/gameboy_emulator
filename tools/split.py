with open("gold03_ret4.log") as f:
  lines = f.readlines()

for i, line in enumerate(lines):
  if line.strip() == "** PRESS START **":
    lines = lines[i:]
    break

with open("gold03_ret4.log.s", "w") as f:
  f.writelines(lines)
