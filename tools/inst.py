import json
with open("inst.json") as f:
    inst = json.loads(f.read())

instmap = {}
for k, v in inst["unprefixed"].items():
    instmap[k] = v["mnemonic"] + " " + " ".join([(o["name"] if o["immediate"] else "[" + o["name"] + "]") for o in v["operands"]])

for k, v in inst["cbprefixed"].items():
    instmap["0xCB" + k.replace("0x", "")] = v["mnemonic"] + " " + " ".join([(o["name"] if o["immediate"] else "[" + o["name"] + "]") for o in v["operands"]])

with open("instmap.json", "w") as f:
    f.write(json.dumps(instmap, indent=2))
