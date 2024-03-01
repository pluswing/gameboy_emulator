import httpclient
import htmlparser
import streams
import xmltree
import nimquery
import strutils
import strformat

type OpsCode = object
  code: string
  name: string
  args: seq[string]
  bytes: int
  cycles: string # FIXME
  flags: string

let url = "https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"

let client = newHttpClient()
let response = client.get(url)
let html = response.body.newStringStream().parseHtml()

let tables = html.querySelectorAll("[class=\"withborder\"]")
let noPrefixedOps = tables[0].querySelectorAll("tr")
# let prefixedOps = tables[1].querySelectorAll("tr")

var ops = newSeq[OpsCode]()

for i, tr in noPrefixedOps:
  if i == 0:
    continue
  let tds = tr.querySelectorAll("td")
  for j, td in tds:
    if j == 0:
      continue
    let code = ((i - 1) shl 8) + (j - 1)
    var children = newSeq[string]()
    for c in td.items():
      children.add(c.innerText)
    let names = split(children[0], " ")
    let name = names[0]
    let args = if len(names) == 1: @[] else: split(names[1], ",")
    let bytes = children[2]
    let cycles = children[5]
    let flags = children[7]

    ops.add(OpsCode(
      code: fmt"{code:#04X}",
      name: name,
      args: args,
      bytes: parseInt(bytes),
      cycles: cycles,
      flags: flags
    ))
  break

echo(ops)
