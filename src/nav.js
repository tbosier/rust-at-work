/* Finding your way: a slim bar naming the section you are in, a contents
   panel with progress, the spot you reached last time, questions to retry,
   and progress files for moving between browsers. Runs after engine.js and
   uses its saved state and loaders. */
(() => {
  const header = document.querySelector("header.top");
  const main = document.querySelector("main");
  if (!header || !main) return;
  // U+203A. Every face in --head and --body (Gill Sans, Noto Sans, Liberation
  // Sans, Charter, Noto Serif, Liberation Serif) has it.
  const SEP = " › ";
  const SPOT_KEY = "rust_at_work.spot"; // "t3-warm-up-drills": the last heading reached

  const clean = el => el.textContent.replace(/\s+/g, " ").trim();
  const shortName = text => text.split(":")[0];
  const slug = text => text.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, 40) || "part";
  const plural = (n, one, many) => `${n} ${n === 1 ? one : many}`;
  const listText = items => items.length < 2 ? items.join("") : `${items.slice(0, -1).join(", ")} and ${items[items.length - 1]}`;

  // Sections (h2) and their parts (h3 and h4), in page order. A part without an id
  // gets one made from its section and its text, so a saved spot can name it.
  const sections = [];
  const marks = [];
  for (const h of main.querySelectorAll("h2, h3, h4")) {
    if (header.contains(h)) continue;
    const isSection = h.tagName === "H2";
    const sec = sections[sections.length - 1];
    if (!isSection && !sec) continue;
    if (!h.id) {
      const base = isSection ? slug(clean(h)) : `${sec.id}-${slug(clean(h))}`;
      let id = base;
      for (let n = 2; document.getElementById(id); n++) id = `${base}-${n}`;
      h.id = id;
    }
    if (isSection) {
      const s = { h, id: h.id, text: clean(h), boxes: [], quiz: null };
      sections.push(s);
      marks.push({ h, id: h.id, sec: s, label: s.text });
    } else {
      marks.push({ h, id: h.id, sec, label: shortName(sec.text) + SEP + clean(h) });
    }
  }
  const markById = new Map(marks.map(m => [m.id, m]));
  const sectionOf = el => {
    let found = null;
    for (const s of sections) {
      if (s.h.compareDocumentPosition(el) & Node.DOCUMENT_POSITION_FOLLOWING) found = s; else break;
    }
    return found;
  };
  checkboxes.forEach(b => { const s = sectionOf(b); if (s) s.boxes.push(b); });
  quizRoots.forEach(r => {
    const s = sectionOf(r);
    if (s && !s.quiz && r.dataset.quiz !== "exam") s.quiz = r.dataset.quiz;
  });

  function progressText(s) {
    const parts = [];
    if (s.boxes.length) parts.push(`${s.boxes.filter(b => b.checked).length} of ${s.boxes.length} criteria`);
    if (s.quiz) {
      const qs = QUIZZES[s.quiz].questions;
      let answered = 0, right = 0;
      qs.forEach((q, i) => {
        const id = `${s.quiz}-${i + 1}`;
        if (!Object.hasOwn(answers, id)) return;
        answered++;
        if (q.options[answers[id]].ok) right++;
      });
      parts.push(answered === 0 ? "quiz not started" : right === qs.length ? "quiz all correct" : `quiz ${right} of ${qs.length} correct`);
    }
    return parts.join(", ");
  }
  const progSpans = [];
  function addProgress(a, s) {
    if (!s.boxes.length && !s.quiz) return;
    const span = document.createElement("span");
    span.className = "prog";
    a.after(span);
    progSpans.push([span, s]);
  }
  const refreshProgress = () => progSpans.forEach(([span, s]) => { span.textContent = progressText(s); });
  document.querySelectorAll("nav.toc a[href^='#']").forEach(a => {
    const s = sections.find(x => x.id === a.getAttribute("href").slice(1));
    if (s) addProgress(a, s);
  });

  // The bar and its panel. Built from a fixed template; page text goes in
  // with textContent only.
  const bar = document.createElement("nav");
  bar.className = "where-bar";
  bar.setAttribute("aria-label", "Where you are");
  bar.innerHTML = `<button type="button" class="where" aria-expanded="false" aria-controls="where-panel"><span class="where-loc"></span><span class="where-open">Contents</span></button>
<div class="where-panel" id="where-panel" hidden>
  <p class="pickup" hidden><a href="#"></a></p>
  <ol class="where-list"></ol>
  <div class="carry">
    <p>Progress is saved per browser and per address (a copy opened from file:// and one from localhost keep separate progress), so save it to a file to move it between them.</p>
    <div class="carry-btns">
      <button type="button" class="btn btn-quiet carry-save">Save progress to a file</button>
      <button type="button" class="btn btn-quiet carry-load">Load progress from a file</button>
    </div>
    <input type="file" class="carry-file" accept=".json,application/json" hidden>
    <p class="carry-msg" role="status"></p>
  </div>
</div>`;
  document.body.prepend(bar);
  const btn = bar.querySelector(".where");
  const loc = bar.querySelector(".where-loc");
  const panel = bar.querySelector(".where-panel");
  const panelPickup = panel.querySelector(".pickup");
  const list = panel.querySelector(".where-list");
  const panelLinks = new Map();
  sections.forEach(s => {
    const li = document.createElement("li");
    const a = document.createElement("a");
    a.href = `#${s.id}`;
    a.textContent = s.text;
    const here = document.createElement("span");
    here.className = "where-here";
    here.setAttribute("aria-hidden", "true"); // aria-current says it to screen readers
    here.textContent = " (you are here)";
    here.hidden = true;
    li.append(a, here);
    list.append(li);
    addProgress(here, s);
    panelLinks.set(s.id, a);
  });

  let current = null;
  let isOpen = false;
  let ticking = false;

  // Which heading the reader is in: the last one above a line just under the bar.
  const rem = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
  function findCurrent() {
    const line = 6 * rem;
    let cur = null;
    for (const m of marks) {
      const rects = m.h.getClientRects();
      if (!rects.length) continue; // inside a closed <details>
      if (rects[0].top <= line) cur = m; else break;
    }
    return cur;
  }
  function markCurrent() {
    const sid = current ? current.sec.id : null;
    panelLinks.forEach((a, id) => {
      const on = id === sid;
      if (on) a.setAttribute("aria-current", "location"); else a.removeAttribute("aria-current");
      a.nextElementSibling.hidden = !on;
    });
  }

  // The spot from the last visit, if it is still a heading on this page.
  const parseSpot = v => (typeof v === "string" && markById.has(v) ? v : null);
  let resumeId = parseSpot(readStored(SPOT_KEY));
  let spot = resumeId;

  function update() {
    ticking = false;
    const cur = findCurrent();
    if (cur === current) return;
    current = cur;
    loc.textContent = cur ? cur.label : document.title;
    if (cur && cur.id !== spot) { spot = cur.id; writeStored(SPOT_KEY, spot); }
    if (isOpen) markCurrent();
  }
  const onScroll = () => { if (!ticking) { ticking = true; requestAnimationFrame(update); } };
  addEventListener("scroll", onScroll, { passive: true });
  addEventListener("resize", onScroll);
  loc.textContent = document.title;

  // The bar stays out of the way while the header is on screen, unless it
  // is open or holds focus.
  let headerOnScreen = header.getBoundingClientRect().bottom > 0;
  const syncBar = () => { bar.hidden = headerOnScreen && !isOpen && !bar.contains(document.activeElement); };
  if ("IntersectionObserver" in window) {
    new IntersectionObserver(es => { headerOnScreen = es[es.length - 1].isIntersecting; syncBar(); }).observe(header);
  } else {
    headerOnScreen = false;
  }
  syncBar();

  function openPanel() {
    isOpen = true;
    update();
    markCurrent();
    renderPickups();
    panel.hidden = false;
    btn.setAttribute("aria-expanded", "true");
    syncBar();
  }
  function closePanel(returnFocus) {
    if (!isOpen) return;
    isOpen = false;
    panel.hidden = true;
    btn.setAttribute("aria-expanded", "false");
    if (returnFocus) btn.focus();
    syncBar();
  }
  btn.addEventListener("click", () => (isOpen ? closePanel(false) : openPanel()));
  document.addEventListener("keydown", e => { if (e.key === "Escape" && isOpen) closePanel(true); });
  document.addEventListener("pointerdown", e => { if (isOpen && !bar.contains(e.target)) closePanel(false); });
  bar.addEventListener("focusout", e => {
    if (isOpen && e.relatedTarget && !bar.contains(e.relatedTarget)) closePanel(false);
    setTimeout(syncBar);
  });

  // Following a link from the panel closes it and puts focus on the target,
  // so the next Tab carries on from there.
  function goTo(target) {
    if (location.hash !== `#${target.id}`) location.hash = target.id;
    target.scrollIntoView({ block: "start" });
    if (!target.hasAttribute("tabindex")) { target.tabIndex = -1; target.classList.add("nav-target"); }
    target.focus({ preventScroll: true });
  }
  panel.addEventListener("click", e => {
    const a = e.target.closest("a[href^='#']");
    const target = a && document.getElementById(a.getAttribute("href").slice(1));
    if (!target) return;
    e.preventDefault();
    closePanel(false);
    goTo(target);
  });

  // In the header: "Pick up where you left off" and the questions to retry.
  const back = document.createElement("div");
  back.className = "return";
  const backBody = document.createElement("div");
  const backMsg = document.createElement("p");
  backMsg.className = "nav-sr";
  backMsg.setAttribute("role", "status");
  back.append(backBody, backMsg);
  const h1 = header.querySelector("h1");
  if (h1) h1.after(back); else header.prepend(back);

  function fillPickup(p) {
    const m = resumeId && markById.get(resumeId);
    p.hidden = !m;
    if (!m) return;
    const a = p.querySelector("a");
    a.href = `#${m.id}`;
    a.textContent = `Pick up where you left off: ${m.label}`;
  }
  const headerPickup = document.createElement("p");
  headerPickup.className = "pickup";
  headerPickup.append(document.createElement("a"));
  function renderPickups() { fillPickup(headerPickup); fillPickup(panelPickup); }

  function questionName(qid) {
    const { fs } = questionEls.get(qid);
    const s = sectionOf(fs);
    const n = qid.slice(qid.lastIndexOf("-") + 1);
    return `${s ? shortName(s.text) : qid}, question ${n}`;
  }
  // The question as the quiz shows it, minus its number. This copies the
  // page's own quiz markup, never stored data.
  function questionText(qid) {
    const legend = questionEls.get(qid).fs.querySelector("legend").cloneNode(true);
    legend.querySelector(".qn")?.remove();
    const span = document.createElement("span");
    span.append(...legend.childNodes);
    if (span.firstChild && span.firstChild.nodeType === Node.TEXT_NODE) span.firstChild.textContent = span.firstChild.textContent.trimStart();
    return span;
  }
  function retryBox(due) {
    const box = document.createElement("section");
    box.className = "retry";
    box.setAttribute("aria-labelledby", "retry-h");
    const h = document.createElement("p");
    h.className = "retry-h";
    h.id = "retry-h";
    h.textContent = "Questions to retry";
    const note = document.createElement("p");
    note.className = "retry-note";
    note.textContent = "You missed these on the first try two or more days ago. Each one starts blank again, so you answer it from memory.";
    const ul = document.createElement("ul");
    due.forEach(qid => {
      const name = questionName(qid);
      const li = document.createElement("li");
      const a = document.createElement("a");
      a.href = `#q-${qid}`;
      a.textContent = name;
      const del = document.createElement("button");
      del.type = "button";
      del.className = "linkbtn retry-dismiss";
      del.textContent = "Dismiss";
      del.setAttribute("aria-label", `Dismiss ${name}`);
      del.addEventListener("click", () => {
        const idx = [...back.querySelectorAll(".retry-dismiss")].indexOf(del);
        updateStored(RETRY_KEY, parseRetry, retry, r => { delete r[qid]; });
        renderReturn();
        backMsg.textContent = `Took ${name} off the retry list.`;
        const left = back.querySelectorAll(".retry-dismiss");
        if (left.length) left[Math.min(idx, left.length - 1)].focus();
        else focusQuietly(headerPickup.hidden ? (h1 || header) : headerPickup.querySelector("a"));
      });
      li.append(a, ": ", questionText(qid), " ", del);
      ul.append(li);
    });
    box.append(h, note, ul);
    return box;
  }
  function focusQuietly(el) {
    if (!el.matches("a, button") && !el.hasAttribute("tabindex")) { el.tabIndex = -1; el.classList.add("nav-target"); }
    el.focus();
  }
  function renderReturn() {
    renderPickups();
    backBody.replaceChildren();
    if (!headerPickup.hidden) backBody.append(headerPickup);
    const due = [...questionIds.keys()].filter(id => isDue(id) && questionEls.has(id));
    if (due.length) backBody.append(retryBox(due));
  }

  // Progress files. Export writes what this page tracks (no theme or other
  // preferences); import runs every value through the same loaders as storage.
  const carryMsg = panel.querySelector(".carry-msg");
  const fileInput = panel.querySelector(".carry-file");
  const KEYS = [CHECKS_KEY, EXAM_KEY, QUIZ_KEY, RETRY_KEY, SPOT_KEY];
  panel.querySelector(".carry-save").addEventListener("click", () => {
    // Read storage again: another open tab may have saved since this one loaded.
    const fresh = (key, parse, mem) => {
      const raw = readFresh(key);
      return raw !== null ? parse(raw) : mem;
    };
    const data = {
      [CHECKS_KEY]: [...fresh(CHECKS_KEY, parseChecks, checked)],
      [QUIZ_KEY]: { ...fresh(QUIZ_KEY, parseAnswers, answers) },
      [RETRY_KEY]: { ...fresh(RETRY_KEY, parseRetry, retry) },
    };
    const exam = loadExam();
    if (exam) data[EXAM_KEY] = exam;
    const lastSpot = parseSpot(readFresh(SPOT_KEY)) || spot;
    if (lastSpot) data[SPOT_KEY] = lastSpot;
    const file = { kind: "rust_at_work progress", version: 1, saved: today(), data };
    const name = `rust_at_work-progress-${today()}.json`;
    const a = document.createElement("a");
    a.href = URL.createObjectURL(new Blob([JSON.stringify(file, null, 2)], { type: "application/json" }));
    a.download = name;
    document.body.append(a);
    a.click();
    a.remove();
    setTimeout(() => URL.revokeObjectURL(a.href), 1000);
    carryMsg.textContent = `Your progress is downloading as ${name}.`;
  });
  panel.querySelector(".carry-load").addEventListener("click", () => fileInput.click());
  fileInput.addEventListener("change", async () => {
    const f = fileInput.files[0];
    fileInput.value = "";
    if (!f) return;
    const notOurs = "That file isn't a progress file from this page, so nothing was loaded.";
    if (f.size > 500000) { carryMsg.textContent = notOurs; return; }
    let parsed = null;
    try { parsed = JSON.parse(await f.text()); } catch (e) { /* handled below */ }
    if (!isPlainObject(parsed) || !isPlainObject(parsed.data)) { carryMsg.textContent = notOurs; return; }
    carryMsg.textContent = importProgress(parsed.data);
  });

  // A key in the file replaces this page's copy only when it has the right
  // shape and at least one valid entry (or is empty on purpose, as an export
  // with nothing ticked is). A key whose entries are all invalid changes nothing.
  function importProgress(data) {
    const has = k => Object.hasOwn(data, k);
    const size = v => (Array.isArray(v) ? v.length : isPlainObject(v) ? Object.keys(v).length : 1);
    const usable = (raw, n) => n > 0 || size(raw) === 0;
    const loaded = [];
    const notes = [];
    let skipped = Object.keys(data).filter(k => !KEYS.includes(k)).length;
    let kept = true;
    let quizReplaced = false;
    let quizAt = -1;
    if (has(CHECKS_KEY)) {
      const raw = data[CHECKS_KEY];
      const s = parseChecks(raw);
      skipped += size(raw) - s.size;
      if (Array.isArray(raw) && usable(raw, s.size)) {
        checked.clear();
        s.forEach(id => checked.add(id));
        checkboxes.forEach(b => { b.checked = checked.has(b.id); });
        kept = writeStored(CHECKS_KEY, [...checked]) && kept;
        loaded.push(plural(s.size, "ticked criterion", "ticked criteria"));
      }
    }
    if (has(QUIZ_KEY)) {
      const raw = data[QUIZ_KEY];
      const fresh = parseAnswers(raw);
      skipped += size(raw) - Object.keys(fresh).length;
      if (isPlainObject(raw) && usable(raw, Object.keys(fresh).length)) {
        replaceAll(answers, fresh);
        kept = writeStored(QUIZ_KEY, answers) && kept;
        quizReplaced = true;
        quizAt = loaded.push("") - 1; // filled in below, after due answers are cleared
      }
    }
    if (has(RETRY_KEY)) {
      const raw = data[RETRY_KEY];
      const fresh = parseRetry(raw);
      skipped += size(raw) - Object.keys(fresh).length;
      if (isPlainObject(raw) && usable(raw, Object.keys(fresh).length)) {
        replaceAll(retry, fresh);
        kept = writeStored(RETRY_KEY, retry) && kept;
        loaded.push(plural(Object.keys(fresh).length, "question to retry", "questions to retry"));
      }
    }
    if (has(EXAM_KEY)) {
      const exam = parseExam(data[EXAM_KEY]);
      if (!exam) skipped++;
      else if (writeStored(EXAM_KEY, exam)) {
        loaded.push("your last exam score");
        const result = document.querySelector('[data-quiz="exam"] .exam-result');
        if (result && !result.querySelector(".score")) result.innerHTML = `<p class="muted">${lastExamText()}</p>`;
      } else notes.push("This browser won't store an exam score, so that wasn't loaded.");
    }
    if (has(SPOT_KEY)) {
      const s = parseSpot(data[SPOT_KEY]);
      if (s) {
        resumeId = spot = s;
        kept = writeStored(SPOT_KEY, s) && kept;
        loaded.push("your place on the page");
      } else skipped++;
    }
    // Imported retry dates can make questions due now, which clears their answers.
    const cleared = askDueCold();
    if (quizReplaced) {
      loaded[quizAt] = plural(Object.keys(answers).length, "quiz answer", "quiz answers");
      questionEls.forEach((_, qid) => showSaved(qid));
    } else {
      cleared.forEach(showSaved);
    }
    if (cleared.length) notes.unshift(`Cleared ${plural(cleared.length, "answer", "answers")} due for retry, so you answer ${cleared.length === 1 ? "it" : "them"} from memory.`);
    refreshProgress();
    renderReturn();
    if (skipped > 0) notes.push(`Left out ${plural(skipped, "entry", "entries")} that didn't match this page.`);
    if (!loaded.length) return ["Nothing in that file matched this page, so nothing changed.", ...notes.filter(n => !n.startsWith("Left out"))].join(" ");
    if (!kept) notes.push("This browser won't keep it after the page closes, so load the file again next time.");
    return [`Loaded ${listText(loaded)}.`, ...notes].join(" ");
  }

  document.addEventListener("change", e => { if (e.target.matches && e.target.matches("input.crit-box")) refreshProgress(); });
  document.addEventListener("rw-progress", () => { refreshProgress(); renderReturn(); });
  refreshProgress();
  renderReturn();
  update();
  addEventListener("load", update);
})();
