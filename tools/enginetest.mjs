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
// ---- Glossary: term links, the popover, and where-it's-taught links ----
{
  const badTerms = await p.evaluate(() => [...document.querySelectorAll("a.term")]
    .map(a => a.getAttribute("href"))
    .filter(h => { const t = document.getElementById(h.slice(1)); return !t || t.tagName !== "DT" || !t.closest("#glossary"); }));
  const nTerms = await p.locator("a.term").count();
  ok(nTerms > 0 && badTerms.length === 0, `all ${nTerms} term links target a glossary entry ${badTerms.join(",")}`);
  const badTaught = await p.evaluate(() => [...document.querySelectorAll("#glossary dd")]
    .filter(dd => { const a = dd.querySelectorAll("a.g-taught"); return a.length !== 1 || !document.getElementById(a[0].getAttribute("href").slice(1)); })
    .map(dd => dd.previousElementSibling.id));
  ok(badTaught.length === 0, "every glossary entry has one where-it's-taught link that resolves " + badTaught.join(","));
  const term = p.locator('a.term[href="#g-heap"]').first();
  await term.scrollIntoViewIfNeeded();
  await term.click();
  const pop = p.locator("#g-pop");
  ok(await pop.isVisible(), "clicking a term opens the popover");
  const want = await p.evaluate(() => document.querySelector("#g-heap + dd .g-def").textContent.trim());
  ok((await pop.locator(".g-pop-def").innerText()).trim() === want, "popover shows the glossary definition");
  ok(await p.evaluate(() => document.activeElement.id === "g-pop" && location.hash === ""), "popover takes focus and the page stays put");
  ok(await term.getAttribute("aria-expanded") === "true", "term says it is expanded");
  const tHref = await pop.locator(".g-pop-taught").getAttribute("href");
  ok(tHref === await p.evaluate(() => document.querySelector("#g-heap + dd a.g-taught").getAttribute("href")), "popover links to where it's taught");
  ok(await pop.locator(".g-pop-entry").getAttribute("href") === "#g-heap", "popover links to the glossary entry");
  await p.keyboard.press("Escape");
  ok(!(await pop.isVisible()), "Escape closes the popover");
  ok(await p.evaluate(() => document.activeElement.matches('a.term[href="#g-heap"]')), "focus returns to the term");
  // Keyboard: Enter on a focused term opens it; a click on plain text closes it.
  await term.focus();
  await p.keyboard.press("Enter");
  ok(await pop.isVisible(), "Enter on a term opens the popover");
  await p.mouse.click(5, 5);
  ok(!(await pop.isVisible()), "a click outside closes the popover");
  // Shift+Tab back to the term keeps it open; once more past it closes it.
  await term.focus();
  await p.keyboard.press("Enter");
  await p.keyboard.press("Shift+Tab");
  ok(await pop.isVisible() && await p.evaluate(() => document.activeElement.matches('a.term[href="#g-heap"]')), "Shift+Tab from the popover lands on its term, still open");
  await p.keyboard.press("Shift+Tab");
  ok(!(await pop.isVisible()) && await term.getAttribute("aria-expanded") === "false", "tabbing past the term closes the popover");
  // Phone width: the popover stays inside the viewport for every term.
  const phone = await ctx.newPage();
  await phone.setViewportSize({ width: 390, height: 844 });
  await phone.goto(url);
  const outside = await phone.evaluate(() => {
    const bad = [];
    // Terms inside a closed hint aren't rendered, so open every hint first.
    document.querySelectorAll("details").forEach(d => { d.open = true; });
    for (const a of document.querySelectorAll("a.term")) {
      a.scrollIntoView({ block: "center" });
      a.click();
      const r = document.getElementById("g-pop").getBoundingClientRect();
      if (r.left < 0 || r.top < 0 || r.right > document.documentElement.clientWidth || r.bottom > innerHeight) bad.push(a.getAttribute("href"));
      a.click();
    }
    return bad;
  });
  ok(outside.length === 0, "popover stays inside a 390px viewport " + outside.slice(0, 5).join(","));
  ok(await phone.evaluate(() => document.documentElement.scrollWidth <= 390), "no horizontal overflow at 390px with the popover");
  await phone.close();
}
ok(errs.length === 0, "no page errors " + errs.join(" | "));
await b.close(); srv.close();
process.exit(fails ? 1 : 0);
