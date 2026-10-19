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
// ---- feat-render: syntax colors, compiler output, theme switch, copy buttons ----
{
  const t = await p.evaluate(() => {
    const rs = [...document.querySelectorAll("pre.rs")];
    const cmd = [...document.querySelectorAll("pre.cmd")];
    return {
      rs: rs.length, rsAllSpans: rs.every(e => e.querySelector("span[class^=sx-]")),
      cmdSpans: cmd.some(e => e.querySelector("span")),
      outSpans: [...document.querySelectorAll("pre.out:not(.cc)")].some(e => e.querySelector("span")),
      cc: document.querySelectorAll("pre.cc").length,
      ccErr: [...document.querySelectorAll("pre.cc")].every(e => e.querySelector(".cc-err")),
      cmds: cmd.length, prompts: document.querySelectorAll("pre.prompt").length,
      cmdCopy: cmd.every(e => e.nextElementSibling?.matches(".copy-bar") && e.nextElementSibling.querySelector("button.copy")),
      promptCopy: [...document.querySelectorAll("pre.prompt")].every(e => e.nextElementSibling?.querySelector("button.copy")),
      rsCopy: rs.some(e => e.nextElementSibling?.matches(".copy-bar")),
      buttons: document.querySelectorAll("button.copy").length,
    };
  });
  ok(t.rs > 100 && t.rsAllSpans, `every Rust block (${t.rs}) has token spans`);
  ok(!t.cmdSpans && !t.outSpans, "shell and program output blocks have no token spans");
  ok(t.cc > 20 && t.ccErr, `compiler output blocks (${t.cc}) color their error lines`);
  ok(t.cmdCopy && t.promptCopy && t.buttons === t.cmds + t.prompts, `copy buttons on ${t.cmds} shell and ${t.prompts} prompt blocks`);
  ok(!t.rsCopy, "no copy button on Rust blocks");
  // Copying: the clipboard gets the commands, and the status line says so.
  await ctx.grantPermissions(["clipboard-read", "clipboard-write"]);
  const firstCmd = p.locator("pre.cmd").first();
  await p.locator("pre.cmd + .copy-bar button.copy").first().click();
  await p.waitForFunction(() => document.querySelector(".copy-status")?.textContent === "Copied.");
  const clip = await p.evaluate(() => navigator.clipboard.readText());
  ok(clip === await firstCmd.evaluate(e => e.textContent), "copy puts the commands on the clipboard");
  const bname = await p.locator("button.copy").first().evaluate(b => b.textContent);
  ok(/^Copy (command|commands|prompt)$/.test(bname), "copy button names itself with its visible label");
  // Without a clipboard the button selects the block instead.
  await p.evaluate(() => { Object.defineProperty(navigator, "clipboard", { value: undefined, configurable: true }); });
  await p.locator("pre.prompt + .copy-bar button.copy").first().click();
  await p.waitForFunction(() => [...document.querySelectorAll(".copy-status")].some(s => s.textContent.startsWith("Selected.")));
  const selected = await p.evaluate(() => window.getSelection().toString() === document.querySelector("pre.prompt").textContent);
  ok(selected, "no clipboard: the block is selected and the reader told to press Ctrl+C");
  // Theme switch: starts on System, a pick persists, garbage falls back.
  await p.evaluate(() => localStorage.removeItem("rust_at_work.theme"));
  await p.reload();
  ok(await p.locator('fieldset.theme input[value="system"]').isChecked(), "theme starts on System");
  ok(await p.evaluate(() => !document.documentElement.hasAttribute("data-theme")), "System leaves data-theme unset");
  await p.locator('fieldset.theme label:has(input[value="dark"])').click();
  ok(await p.evaluate(() => document.documentElement.dataset.theme === "dark" && localStorage.getItem("rust_at_work.theme") === "dark"), "picking Dark applies and saves it");
  const darkBg = await p.evaluate(() => getComputedStyle(document.body).backgroundColor);
  ok(darkBg === "rgb(22, 26, 30)", "Dark paints the dark paper " + darkBg);
  await p.reload();
  ok(await p.evaluate(() => document.documentElement.dataset.theme === "dark"), "Dark survives reload");
  ok(await p.locator('fieldset.theme input[value="dark"]').isChecked(), "Dark radio checked after reload");
  await p.evaluate(() => localStorage.setItem("rust_at_work.theme", '"><img src=x onerror=alert(1)>'));
  await p.reload();
  ok(await p.evaluate(() => !document.documentElement.hasAttribute("data-theme")), "garbage theme falls back to System");
  ok(await p.locator('fieldset.theme input[value="system"]').isChecked(), "garbage theme checks the System radio");
  // A system set to dark, with Light picked, stays light.
  await p.emulateMedia({ colorScheme: "dark" });
  await p.evaluate(() => localStorage.removeItem("rust_at_work.theme"));
  await p.reload();
  ok(await p.evaluate(() => getComputedStyle(document.body).backgroundColor) === "rgb(22, 26, 30)", "System follows a dark system");
  await p.locator('fieldset.theme label:has(input[value="light"])').click();
  ok(await p.evaluate(() => getComputedStyle(document.body).backgroundColor) === "rgb(247, 247, 244)", "Light overrides a dark system");
  await p.emulateMedia({ colorScheme: "light" });
  await p.evaluate(() => localStorage.removeItem("rust_at_work.theme"));
}
// ---- end feat-render ----
ok(errs.length === 0, "no page errors " + errs.join(" | "));
await b.close(); srv.close();
process.exit(fails ? 1 : 0);
