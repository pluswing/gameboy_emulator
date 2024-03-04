import httpclient
import htmlparser
import streams
import xmltree
import nimquery
import strutils
import strformat
import sugar

type
  FlagValue = enum
    NO_CHANGE,
    CHANGE,
    FORCE_TRUE,
    FORCE_FALSE,

  Flags = object
    zero: FlagValue
    subtract: FlagValue
    half_carry: FlagValue
    carry: FlagValue

  OpsCode = object
    code: string
    name: string
    args: seq[string]
    bytes: int
    cycles: seq[int]
    flags: Flags

let url = "https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"

let client = newHttpClient()
let response = client.get(url)
let html = response.body.newStringStream().parseHtml()

let tables = html.querySelectorAll("[class=\"withborder\"]")
let noPrefixedOps = tables[0].querySelectorAll("tr")
# let prefixedOps = tables[1].querySelectorAll("tr")

var operations = newSeq[OpsCode]()

proc flag_value(v: string): FlagValue =
  if v == "1":
    return FlagValue.FORCE_TRUE
  elif v == "0":
    return FlagValue.FORCE_FALSE
  elif v == "-":
    return FlagValue.NO_CHANGE
  else:
    return FlagValue.CHANGE


for i, tr in noPrefixedOps:
  if i == 0:
    continue
  let tds = tr.querySelectorAll("td")
  for j, td in tds:
    if j == 0:
      continue
    let code = ((i - 1) shl 4) + (j - 1)

    var children = collect(newSeq):
      for c in td.items(): c.innerText

    if children.len != 8:
      continue

    let names = children[0].split(" ")
    let name = names[0]
    let args = if names.len == 1: @[] else: names[1].split(",")
    let bytes = children[2]
    let cycles = collect(newSeq):
      for c in children[5].split("/"): c.parseInt
    let raw_flags = children[7].split(" ")
    let flags = Flags(
      zero: flag_value(raw_flags[0]),
      subtract: flag_value(raw_flags[1]),
      half_carry: flag_value(raw_flags[2]),
      carry: flag_value(raw_flags[3]),
    )

    operations.add(OpsCode(
      code: fmt"{code:#04X}",
      name: name,
      args: args,
      bytes: bytes.parseInt,
      cycles: cycles,
      flags: flags
    ))

for op in operations:
  echo(fmt"{op.code} => Some(Instruction::{op.name}(...)),")
