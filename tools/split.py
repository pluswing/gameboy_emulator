with open("gold01.shutdown.log") as f:
  lines = f.readlines()

for i, line in enumerate(lines):
  if line.strip() == "** PRESS START **":
    lines = lines[i:]
    break

with open("gold01.shutdown.log.s", "w") as f:
  f.writelines(lines)
