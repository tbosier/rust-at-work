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
// ---- Navigation and progress (src/nav.js): where-am-I bar, contents panel,
// pick-up spot, saved quiz answers, progress, retry list, export and import.
{
  const S = k => p.evaluate(k => localStorage.getItem(k), k);
  const rightOf = (id, i) => p.evaluate(([id, i]) => QUIZZES[id].questions[i].options.findIndex(o => o.ok), [id, i]);
  const wrongOf = (id, i) => p.evaluate(([id, i]) => QUIZZES[id].questions[i].options.findIndex(o => !o.ok), [id, i]);
  const answer = async (id, i, opt) => {
    const f = p.locator(`#q-${id}-${i + 1}`);
    await f.locator("input").nth(opt).check();
    await f.locator("button.btn").click();
  };
  const settle = () => p.evaluate(() => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))));
  // Quiz answers persist: the earlier tests checked every question right, then t1 q1 wrong.
  await p.reload();
  ok(await p.locator("#q-t3-4").count() === 1 && await p.locator("#q-exam-1").count() === 1, "quiz fieldsets have stable ids");
  ok((await p.locator("#q-t2-1 .feedback").innerText()).includes("Correct"), "checked answer and verdict survive reload");
  ok((await p.locator("#q-t1-1 .feedback").innerText()).includes("Not quite") && await p.locator("#q-t1-1 input").nth(wrong).isChecked(), "a wrong last answer is restored as wrong");
  ok(await p.locator('[data-quiz="exam"] input:checked').count() === 0, "exam answers are not restored");
  await p.evaluate(() => localStorage.setItem("rust_at_work.quiz", '{"t1-1":99,"<img src=x onerror=alert(1)>":1,"t1-2":"1","__proto__":2}'));
  await p.reload();
  ok(await p.locator('[data-quiz="t1"] input:checked').count() === 0, "garbage quiz answers are dropped");
  // Progress in the header contents.
  await p.evaluate(() => {
    localStorage.setItem("rust_at_work.checks", '["c-t3-1","c-t3-2"]');
    const a = {}; QUIZZES.t3.questions.forEach((q, i) => { a[`t3-${i + 1}`] = q.options.findIndex(o => o.ok); });
    a["t2-1"] = QUIZZES.t2.questions[0].options.findIndex(o => !o.ok);
    localStorage.setItem("rust_at_work.quiz", JSON.stringify(a));
  });
  await p.reload();
  const nT3 = await p.locator("#t3 ~ ul.criteria input.crit-box[id^='c-t3-']").count();
  const tocProg = id => p.locator(`nav.toc a[href="#${id}"] + .prog`).innerText();
  ok(await tocProg("t3") === `2 of ${nT3} criteria, quiz all correct`, "toc shows t3 criteria and a full quiz: " + await tocProg("t3"));
  ok((await tocProg("t2")).endsWith("quiz 0 of 6 correct"), "toc shows a partly answered quiz: " + await tocProg("t2"));
  ok((await tocProg("t4")).endsWith("quiz not started"), "toc shows an untouched quiz");
  await p.locator("#c-t3-3").check();
  ok((await tocProg("t3")).startsWith(`3 of ${nT3}`), "toc progress updates when a criterion is ticked");
  // The bar: hidden over the header, names the h2 and h3 once scrolled.
  await p.evaluate(() => scrollTo(0, 0)); await settle();
  ok(await p.locator(".where-bar").isHidden(), "bar hidden while the header is on screen");
  await p.evaluate(() => [...document.querySelectorAll("#t3 ~ h3")].find(h => h.textContent === "Warm-up drills").scrollIntoView()); await settle();
  ok(await p.locator(".where-bar").isVisible(), "bar shows once the header is gone");
  ok(await p.locator(".where-loc").innerText() === "Ticket 3 › Warm-up drills", "bar names section and part: " + await p.locator(".where-loc").innerText());
  ok(await S("rust_at_work.spot") === '"t3-warm-up-drills"', "reading spot saved: " + await S("rust_at_work.spot"));
  // Contents panel: opens, marks the current section, Escape closes and returns focus.
  await p.locator(".where").click();
  ok(await p.locator("#where-panel").isVisible() && await p.locator(".where").getAttribute("aria-expanded") === "true", "panel opens");
  ok(await p.locator('#where-panel a[aria-current]').getAttribute("href") === "#t3", "panel marks the current section");
  ok(await p.locator("#where-panel .where-list li").count() === await p.locator("main > h2[id]").count(), "panel lists every h2");
  await p.locator("#where-panel .where-list a").first().focus();
  await p.keyboard.press("Escape");
  ok(await p.locator("#where-panel").isHidden() && await p.evaluate(() => document.activeElement.classList.contains("where")), "Escape closes the panel and focus returns");
  await p.locator(".where").click();
  await p.locator('#where-panel a[href="#t4"]').click();
  ok(await p.locator("#where-panel").isHidden() && await p.evaluate(() => document.activeElement.id === "t4"), "panel link closes the panel and focuses the section");
  const t4top = await p.evaluate(() => document.getElementById("t4").getBoundingClientRect().top);
  const barBottom = await p.evaluate(() => document.querySelector(".where-bar").getBoundingClientRect().bottom);
  ok(t4top >= barBottom, `anchor jump lands below the bar (${t4top} >= ${barBottom})`);
  await p.locator(".where").focus();
  await p.keyboard.press("Enter");
  await p.locator('#where-panel a[href="#t5"]').focus();
  await p.keyboard.press("Enter");
  ok(await p.evaluate(() => document.activeElement.id === "t5" && getComputedStyle(document.activeElement).outlineStyle !== "none"), "keyboard jump shows a focus ring on the section");
  // Pick up where you left off, from the spot saved last visit.
  await settle();
  await p.evaluate(() => localStorage.setItem("rust_at_work.spot", '"t3-warm-up-drills"'));
  await p.goto(url);
  ok(await p.locator("header .pickup a").innerText() === "Pick up where you left off: Ticket 3 › Warm-up drills", "pick-up link in the header");
  ok(await p.locator("header .pickup a").getAttribute("href") === "#t3-warm-up-drills", "pick-up link target");
  await p.evaluate(() => localStorage.setItem("rust_at_work.spot", '"<b>nope</b>"'));
  await p.goto(url);
  ok(await p.locator("header .pickup").count() === 0, "unknown saved spot shows no pick-up link");
  // Phone width: the bar stays one line.
  await p.setViewportSize({ width: 390, height: 844 });
  await p.evaluate(() => document.getElementById("t4").scrollIntoView()); await settle();
  const barH = await p.evaluate(() => document.querySelector(".where").getBoundingClientRect().height);
  ok(barH < 50, "bar is one line at phone width: " + barH);
  ok(await p.evaluate(() => document.documentElement.scrollWidth) <= 390, "no overflow at phone width");
  await p.setViewportSize({ width: 1280, height: 720 });
  // Retry list, with a faked date.
  await p.evaluate(() => { localStorage.removeItem("rust_at_work.quiz"); localStorage.removeItem("rust_at_work.retry"); });
  await p.goto(url + "?today=2026-10-10");
  await answer("t2", 1, await wrongOf("t2", 1));          // wrong first: recorded
  await answer("t2", 2, await rightOf("t2", 2));          // right first, then wrong: not recorded
  await answer("t2", 2, await wrongOf("t2", 2));
  await answer("t2", 3, await wrongOf("t2", 3));
  await answer("t2", 3, await rightOf("t2", 3));          // wrong first, then right: still recorded
  ok(await S("rust_at_work.retry") === '{"t2-2":"2026-10-10","t2-4":"2026-10-10"}', "only wrong first checks are recorded: " + await S("rust_at_work.retry"));
  await p.goto(url + "?today=2026-10-11");
  ok(await p.locator(".retry").count() === 0, "no retry list before two days");
  ok(await p.locator("#q-t2-2 input:checked").count() === 1, "not-yet-due answer is kept");
  await p.goto(url + "?today=2026-10-12");
  ok(await p.locator('.retry a[href="#q-t2-2"]').count() === 1 && await p.locator('.retry a[href="#q-t2-4"]').count() === 1, "due questions listed with links");
  ok(await p.locator("#q-t2-2 input:checked").count() === 0 && await p.locator("#q-t2-4 input:checked").count() === 0, "due questions are asked cold");
  ok(await p.locator("#q-t2-3 input:checked").count() === 1, "other answers untouched");
  await answer("t2", 1, await rightOf("t2", 1));
  ok(await p.locator('.retry a[href="#q-t2-2"]').count() === 0 && !(await S("rust_at_work.retry")).includes("t2-2"), "right on first check removes it");
  await p.locator(".retry .retry-dismiss").first().click();
  ok(await p.locator(".retry").count() === 0 && await S("rust_at_work.retry") === "{}", "dismiss removes an entry");
  await p.goto(url + "?today=banana");
  ok(await p.evaluate(() => FAKE_TODAY === null && /^\d{4}-\d{2}-\d{2}$/.test(today())), "a bad ?today is ignored");
  // A future date (from a faked ?today) is dropped, so it can't hide a retry for years.
  await p.evaluate(() => localStorage.setItem("rust_at_work.retry", '{"t2-2":"2999-01-01"}'));
  await p.goto(url + "?today=2026-10-12");
  ok(await p.evaluate(() => !Object.hasOwn(retry, "t2-2")), "future retry dates are dropped");
  // A second tab's answers survive this tab's next check.
  await p.evaluate(() => localStorage.setItem("rust_at_work.quiz", JSON.stringify({ ...JSON.parse(localStorage.getItem("rust_at_work.quiz")), "t5-1": 0 })));
  await answer("t5", 1, await rightOf("t5", 1));
  ok(JSON.parse(await S("rust_at_work.quiz"))["t5-1"] === 0, "an answer saved by another tab is kept");
  // Export and import.
  await p.evaluate(() => {
    localStorage.setItem("rust_at_work.theme", "light");
    localStorage.setItem("rust_at_work.checks", '["c-t1-1"]');
  });
  await p.goto(url);
  await p.evaluate(() => document.getElementById("t1").scrollIntoView()); await settle();
  await p.locator(".where").click();
  const [dl] = await Promise.all([p.waitForEvent("download"), p.locator(".carry-save").click()]);
  const exported = JSON.parse(fs.readFileSync(await dl.path(), "utf8"));
  ok(JSON.stringify(exported.data["rust_at_work.checks"]) === '["c-t1-1"]' && "rust_at_work.quiz" in exported.data && "rust_at_work.part1.exam" in exported.data, "export has checks, quiz and exam");
  ok(!Object.keys(exported.data).some(k => k.includes("theme")), "export leaves out the theme");
  exported.data["rust_at_work.checks"] = ["c-t2-1", "c-t2-2", "bogus"];
  exported.data["rust_at_work.quiz"] = { "t4-1": 0, "t9-9": 1 };
  exported.data["rust_at_work.spot"] = "t2";
  exported.data["rust_at_work.theme"] = "dark";
  await p.locator(".carry-file").setInputFiles({ name: "progress.json", mimeType: "application/json", buffer: Buffer.from(JSON.stringify(exported)) });
  await p.waitForFunction(() => document.querySelector(".carry-msg").textContent.startsWith("Loaded"));
  const msg = await p.locator(".carry-msg").innerText();
  ok(msg.includes("2 ticked criteria") && msg.includes("1 quiz answer") && msg.includes("Left out 3 entries"), "import says what it loaded: " + msg);
  ok(await p.locator("#c-t2-1").isChecked() && !(await p.locator("#c-t1-1").isChecked()), "import applies criteria");
  ok(await p.locator("#q-t4-1 input").first().isChecked(), "import applies quiz answers");
  ok(await S("rust_at_work.spot") === '"t2"' && (await p.locator("header .pickup a").getAttribute("href")) === "#t2", "spot imported and shown");
  ok(await S("rust_at_work.theme") === "light" && (await S("rust_at_work.quiz")) === '{"t4-1":0}', "import stores only valid known data, never the theme");
  ok((await p.locator('#where-panel a[href="#t2"] ~ .prog').innerText()).startsWith("2 of"), "panel shows progress");
  ok(await p.locator("#where-panel .pickup a").getAttribute("href") === "#t2", "panel shows the pick-up link");
  // A parseable file whose entries are all invalid changes nothing, and keeps unchecked selections.
  await p.keyboard.press("Escape");
  await p.locator("#q-t6-1 input").first().check();
  await p.locator(".where").click();
  await p.locator(".carry-file").setInputFiles({ name: "y.json", mimeType: "application/json", buffer: Buffer.from(JSON.stringify({ data: { "rust_at_work.checks": ["bogus"], "rust_at_work.quiz": { "t1-1": 99 }, "rust_at_work.retry": { "t1-1": "banana" }, "unrelated": 1 } })) });
  await p.waitForFunction(() => document.querySelector(".carry-msg").textContent.startsWith("Nothing"));
  ok(await p.locator("#c-t2-1").isChecked() && (await S("rust_at_work.quiz")) === '{"t4-1":0}' && await p.locator("#q-t6-1 input").first().isChecked(), "an all-invalid file keeps existing progress");
  // An imported retry that is due clears its answer, and the message says so.
  await p.locator(".carry-file").setInputFiles({ name: "z.json", mimeType: "application/json", buffer: Buffer.from(JSON.stringify({ data: { "rust_at_work.quiz": { "t4-1": 0, "t4-2": 0 }, "rust_at_work.retry": { "t4-1": "2020-01-01" } } })) });
  await p.waitForFunction(() => document.querySelector(".carry-msg").textContent.startsWith("Loaded"));
  const msg2 = await p.locator(".carry-msg").innerText();
  ok(msg2.includes("1 quiz answer ") && msg2.includes("Cleared 1 answer") && await p.locator("#q-t4-1 input:checked").count() === 0, "due import counts only kept answers: " + msg2);
  await p.locator(".carry-file").setInputFiles({ name: "x.json", mimeType: "application/json", buffer: Buffer.from("not json {") });
  await p.waitForFunction(() => document.querySelector(".carry-msg").textContent.includes("isn't a progress file"));
  ok(await p.locator("#c-t2-1").isChecked(), "a bad file changes nothing");
  await p.reload();
  ok(await p.locator("#c-t2-1").isChecked() && await p.locator("#q-t4-2 input").first().isChecked(), "imported progress survives reload");
}
// ---- end of navigation and progress tests
ok(errs.length === 0, "no page errors " + errs.join(" | "));
await b.close(); srv.close();
process.exit(fails ? 1 : 0);
