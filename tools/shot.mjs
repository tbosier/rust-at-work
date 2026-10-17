// node tools/shot.mjs out-prefix [#anchor ...]: screenshots at desktop and phone width,
// plus checks for console errors and horizontal overflow. Run unsandboxed.
import http from "node:http"; import fs from "node:fs"; import path from "node:path";
import { createRequire } from "node:module";
// playwright-core is not a dependency of this repo. Point PLAYWRIGHT_FROM at any
// package.json whose node_modules has it, or install it next to these tools.
const require = createRequire(process.env.PLAYWRIGHT_FROM ?? import.meta.url);
const { chromium } = require("playwright-core");
const file = path.resolve("dist/rust_at_work.html");
const srv = http.createServer((q, r) => { r.writeHead(200, { "content-type": "text/html" }); r.end(fs.readFileSync(file)); });
await new Promise(ok => srv.listen(0, ok));
const url = `http://127.0.0.1:${srv.address().port}/`;
const [prefix, ...anchors] = process.argv.slice(2);
const b = await chromium.launch({ executablePath: process.env.CHROME ?? "/usr/bin/google-chrome-stable", args: ["--no-sandbox", "--headless=new"] });
for (const [name, w, h] of [["desk", 1280, 900], ["phone", 390, 844]]) {
  const p = await b.newPage({ viewport: { width: w, height: h } });
  const errs = []; p.on("pageerror", e => errs.push(e.message)); p.on("console", m => m.type() === "error" && errs.push(m.text()));
  await p.goto(url, { waitUntil: "load" });
  const sw = await p.evaluate(() => document.documentElement.scrollWidth);
  const wide = await p.evaluate(() => [...document.querySelectorAll("main *")].filter(e => e.getBoundingClientRect().right > document.documentElement.clientWidth + 1 && !e.closest("pre, .table-wrap, .diagram")).slice(0, 5).map(e => e.tagName + "." + e.className + ": " + e.textContent.slice(0, 40)));
  const quizzes = await p.evaluate(() => [...document.querySelectorAll("[data-quiz]")].map(q => q.dataset.quiz + ":" + q.querySelectorAll("fieldset.q").length).join(" "));
  console.log(name, "scrollWidth", sw, "viewport", w, errs.length ? "ERRORS " + errs.join(" | ") : "no console errors");
  console.log("  quizzes", quizzes);
  if (wide.length) console.log("  overflowing:", wide);
  for (const a of anchors) {
    await p.evaluate(id => document.getElementById(id).scrollIntoView(), a.slice(1));
    await p.screenshot({ path: `${prefix}-${name}-${a.slice(1)}.png` });
  }
  await p.close();
}
await b.close(); srv.close();
