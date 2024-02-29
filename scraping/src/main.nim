import httpclient
import htmlparser
import streams
import xmltree
import nimquery
import strutils

let url = "https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"

let client = newHttpClient()
let response = client.get(url)
let html = response.body.newStringStream().parseHtml()

let tables = html.querySelectorAll("[class=\"withborder\"]")
let noPrefixedOps = tables[0].querySelectorAll("tr")
# let prefixedOps = tables[1].querySelectorAll("tr")

for i, tr in noPrefixedOps:
  if i == 0:
    continue
  let tds = tr.querySelectorAll("td")
  for j, td in tds:
    if j == 0:
      continue
    echo(i, j, td.innerText)
