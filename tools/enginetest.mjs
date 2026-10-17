// node tools/enginetest.mjs: exercise quizzes, the exam and the criteria checkboxes. Run unsandboxed.
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
const b = await chromium.launch({ executablePath: process.env.CHROME ?? "/usr/bin/google-chrome-stable", args: ["--no-sandbox", "--headless=new"] });
const ctx = await b.newContext();
const p = await ctx.newPage();
const errs = []; p.on("pageerror", e => errs.push(e.message));
let fails = 0; const ok = (c, m) => { console.log((c ? "ok   " : "FAIL ") + m); if (!c) fails++; };
await p.goto(url);
// Every quiz question has exactly one right answer, and checking it says Correct.
const quizIds = await p.evaluate(() => Object.keys(QUIZZES));
for (const id of quizIds) {
  const bad = await p.evaluate(id => QUIZZES[id].questions.filter(q => q.options.filter(o => o.ok).length !== 1).length, id);
  ok(bad === 0, `${id}: every question has one right answer`);
  if (id === "exam") continue;
  const n = await p.evaluate(id => QUIZZES[id].questions.length, id);
  for (let i = 0; i < n; i++) {
    const right = await p.evaluate(([id, i]) => QUIZZES[id].questions[i].options.findIndex(o => o.ok), [id, i]);
    const fs_ = p.locator(`[data-quiz="${id}"] fieldset.q`).nth(i);
    await fs_.locator("input").nth(right).check();
    await fs_.locator("button.btn").click();
    ok((await fs_.locator(".feedback").innerText()).includes("Correct"), `${id} q${i + 1} right answer marked correct`);
  }
}
// A wrong answer shows its why text.
const fs0 = p.locator('[data-quiz="t1"] fieldset.q').first();
const wrong = await p.evaluate(() => QUIZZES.t1.questions[0].options.findIndex(o => !o.ok));
await fs0.locator("input").nth(wrong).check();
await fs0.locator("button.btn").click();
ok((await fs0.locator(".feedback").innerText()).includes("Not quite"), "wrong answer says Not quite");
// Exam: answer all right, grade, score saved.
const total = await p.evaluate(() => QUIZZES.exam.questions.length);
for (let i = 0; i < total; i++) {
  const right = await p.evaluate(i => QUIZZES.exam.questions[i].options.findIndex(o => o.ok), i);
  await p.locator('[data-quiz="exam"] fieldset.q').nth(i).locator("input").nth(right).check();
}
await p.locator('[data-quiz="exam"] .grade').click();
ok((await p.locator('[data-quiz="exam"] .score').innerText()) === `${total} of ${total}`, "exam full marks");
// Every exam sec points at a real section.
const secs = await p.evaluate(() => QUIZZES.exam.questions.map(q => q.sec).filter(s => !document.getElementById(s) || !SECTION_NAMES[s]));
ok(secs.length === 0, "exam review links resolve " + secs.join(","));
// Criteria persist across reload; garbage in storage is ignored.
const first = p.locator("input.crit-box").first();
await first.check();
await p.reload();
ok(await p.locator("input.crit-box").first().isChecked(), "criteria checkbox survives reload");
await p.evaluate(() => localStorage.setItem("rust_at_work.checks", '{"x":1}'));
await p.reload();
ok(!(await p.locator("input.crit-box").first().isChecked()), "garbage in storage falls back to unchecked");
await p.evaluate(() => localStorage.setItem("rust_at_work.checks", '["c-t1-1","<img src=x onerror=alert(1)>"]'));
await p.reload();
ok(await p.locator("#c-t1-1").isChecked(), "known id restored, unknown ignored");
ok(errs.length === 0, "no page errors " + errs.join(" | "));
await b.close(); srv.close();
process.exit(fails ? 1 : 0);
