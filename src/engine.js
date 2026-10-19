const ICON_OK = '<svg class="ico" viewBox="0 0 16 16" aria-hidden="true"><path d="M3 8.5l3 3 7-7" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/></svg>';
const ICON_BAD = '<svg class="ico" viewBox="0 0 16 16" aria-hidden="true"><path d="M4 4l8 8M12 4l-8 8" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/></svg>';

function renderQuestion(quizId, q, i, isExam) {
  const fs = document.createElement("fieldset");
  fs.className = "q";
  const legend = document.createElement("legend");
  legend.innerHTML = `<span class="qn">${i + 1}.</span> ${q.q}`;
  fs.appendChild(legend);
  if (q.code) {
    const pre = document.createElement("pre");
    pre.textContent = q.code;
    fs.appendChild(pre);
  }
  const opts = document.createElement("div");
  opts.className = "opts";
  q.options.forEach((opt, j) => {
    const label = document.createElement("label");
    label.className = "opt";
    const input = document.createElement("input");
    input.type = "radio";
    input.name = `${quizId}-${i}`;
    input.value = String(j);
    const span = document.createElement("span");
    span.innerHTML = opt.t;
    label.append(input, span);
    opts.appendChild(label);
  });
  fs.appendChild(opts);
  const feedback = document.createElement("div");
  feedback.className = "feedback";
  feedback.setAttribute("aria-live", "polite");
  if (!isExam) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "btn";
    btn.textContent = "Check";
    btn.addEventListener("click", () => checkOne(fs, q, feedback));
    fs.appendChild(btn);
  }
  fs.appendChild(feedback);
  feedback.tabIndex = -1;
  // Stable ids, like q-t3-4, so the retry list can link to a question.
  const qid = `${quizId}-${i + 1}`;
  fs.id = `q-${qid}`;
  // Changing the answer makes any earlier verdict stale: clear it.
  opts.addEventListener("change", () => {
    feedback.className = "feedback";
    feedback.innerHTML = "";
    fs.querySelectorAll(".opt").forEach(l => l.classList.remove("is-ok", "is-bad"));
    if (isExam) markExamStale(fs.closest("[data-quiz]"));
  });
  if (!isExam) {
    fs.dataset.qid = qid;
    questionEls.set(qid, { fs, q, feedback });
    showSaved(qid);
  }
  return fs;
}

function markExamStale(root) {
  const result = root && root.querySelector(".exam-result");
  if (result && result.querySelector(".score")) {
    result.innerHTML = '<p class="muted">You changed an answer. Grade again for an up-to-date score.</p>';
  }
}

function chosenIndex(fs) {
  const picked = fs.querySelector("input:checked");
  return picked ? Number(picked.value) : -1;
}

function markOptions(fs, q, chosen) {
  fs.querySelectorAll(".opt").forEach((label, j) => {
    label.classList.remove("is-ok", "is-bad");
    if (j === chosen) label.classList.add(q.options[j].ok ? "is-ok" : "is-bad");
  });
}

function checkOne(fs, q, feedback) {
  const chosen = chosenIndex(fs);
  if (chosen < 0) {
    feedback.className = "feedback";
    feedback.innerHTML = "<p>Pick an answer first.</p>";
    return;
  }
  showVerdict(fs, q, feedback, chosen);
  recordAnswer(fs.dataset.qid, chosen, q.options[chosen].ok);
}

function showVerdict(fs, q, feedback, chosen) {
  markOptions(fs, q, chosen);
  const opt = q.options[chosen];
  if (opt.ok) {
    feedback.className = "feedback ok";
    feedback.innerHTML = `<p class="verdict">${ICON_OK} Correct.</p><p>${q.explain}</p>`;
  } else {
    feedback.className = "feedback bad";
    feedback.innerHTML = `<p class="verdict">${ICON_BAD} Not quite.</p><p>${opt.why || ""}</p>
      <p class="muted">Try another answer, or <button type="button" class="linkbtn">show the answer</button>.</p>`;
    feedback.querySelector(".linkbtn").addEventListener("click", () => {
      const right = q.options.findIndex(o => o.ok);
      feedback.innerHTML = `<p class="verdict">Answer: ${q.options[right].t}</p><p>${q.explain}</p>`;
      feedback.className = "feedback";
      feedback.focus();
    });
  }
}

/* Exam scores persist per browser. Stored data is untrusted: anything that
   does not look like a real result is ignored. */
const EXAM_KEY = "rust_at_work.part1.exam";
function parseExam(raw) {
  if (!raw || typeof raw !== "object") return null;
  const total = QUIZZES.exam.questions.length;
  const ok = Number.isInteger(raw.score) && raw.score >= 0 && raw.score <= total
    && raw.total === total && typeof raw.date === "string" && /^\d{4}-\d{2}-\d{2}$/.test(raw.date);
  return ok ? { score: raw.score, total: raw.total, date: raw.date } : null;
}
function loadExam() {
  try {
    return parseExam(JSON.parse(localStorage.getItem(EXAM_KEY) || "null"));
  } catch (e) {
    return null;
  }
}
function saveExam(score, total) {
  const d = new Date();
  const date = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  try {
    localStorage.setItem(EXAM_KEY, JSON.stringify({ score, total, date }));
    return true;
  } catch (e) {
    return false; // private window or storage blocked
  }
}
function lastExamText() {
  const last = loadExam();
  return last ? `Last attempt: ${last.score} of ${last.total}, on ${last.date}.` : "No attempts yet in this browser.";
}

function gradeExam(root) {
  const qs = QUIZZES.exam.questions;
  let score = 0;
  const missed = new Set();
  root.querySelectorAll("fieldset.q").forEach((fs, i) => {
    const q = qs[i];
    const chosen = chosenIndex(fs);
    const feedback = fs.querySelector(".feedback");
    const right = q.options.findIndex(o => o.ok);
    const review = `<a href="#${q.sec}">Review: ${SECTION_NAMES[q.sec]}</a>`;
    if (chosen >= 0) markOptions(fs, q, chosen);
    if (chosen === right) {
      score++;
      feedback.className = "feedback ok";
      feedback.innerHTML = `<p class="verdict">${ICON_OK} Correct.</p><p>${q.explain}</p>`;
    } else {
      missed.add(q.sec);
      feedback.className = "feedback bad";
      const lead = chosen < 0 ? "Not answered." : "Not quite.";
      feedback.innerHTML = `<p class="verdict">${ICON_BAD} ${lead}</p><p>Answer: ${q.options[right].t}</p><p>${q.explain}</p><p>${review}</p>`;
    }
  });
  const last = loadExam();
  const previous = last
    ? `Previous attempt: ${last.score} of ${last.total}, on ${last.date}.`
    : "That was your first attempt in this browser.";
  const saved = saveExam(score, qs.length);
  const result = root.querySelector(".exam-result");
  const advice = score === qs.length
    ? "Every one. Come back in a week and do it again cold. If it still holds, you own this lesson."
    : `Reread the sections you missed: ${[...missed].map(s => `<a href="#${s}">${SECTION_NAMES[s]}</a>`).join(", ")}. Then come back in two or three days and take it cold again. Answers that come back after a gap are the ones that stick.`;
  result.innerHTML = `<p class="score">${score} of ${qs.length}</p><p>${advice}</p><p class="muted">${previous} ${saved ? "This score is saved for next time." : "This browser would not save the score, so note it down."}</p>`;
  result.focus();
}

function resetExam(root) {
  root.querySelectorAll("input[type=radio]").forEach(r => { r.checked = false; });
  root.querySelectorAll(".opt").forEach(l => l.classList.remove("is-ok", "is-bad"));
  root.querySelectorAll(".feedback").forEach(f => { f.className = "feedback"; f.innerHTML = ""; });
  root.querySelector(".exam-result").innerHTML = `<p class="muted">${lastExamText()}</p>`;
  const first = root.querySelector("input[type=radio]");
  if (first) first.focus();
  root.scrollIntoView({ block: "start" });
}

/* Practice-quiz answers and the retry list persist per browser, keyed by
   question id ("t3-4"). Stored data is untrusted: only ids of questions on
   this page, option numbers in range and real dates survive. The exam is
   not saved per question. */
const QUIZ_KEY = "rust_at_work.quiz";   // {"t3-4": 2}: the option last checked
const RETRY_KEY = "rust_at_work.retry"; // {"t3-4": "2026-10-16"}: wrong at first check
const RETRY_AFTER_DAYS = 2;
function readStored(key) {
  try { return JSON.parse(localStorage.getItem(key) || "null"); } catch (e) { return null; }
}
function writeStored(key, value) {
  try { localStorage.setItem(key, JSON.stringify(value)); return true; } catch (e) { return false; }
}
const isPlainObject = v => !!v && typeof v === "object" && !Array.isArray(v);
function isoDate(d) {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}
function validDate(s) {
  if (typeof s !== "string" || !/^\d{4}-\d{2}-\d{2}$/.test(s)) return false;
  const [y, m, d] = s.split("-").map(Number);
  const t = new Date(Date.UTC(y, m - 1, d));
  return t.getUTCFullYear() === y && t.getUTCMonth() === m - 1 && t.getUTCDate() === d;
}
const dayNumber = s => { const [y, m, d] = s.split("-").map(Number); return Date.UTC(y, m - 1, d) / 86400000; };
// Tests can fake the date with ?today=YYYY-MM-DD. Only the retry list reads it,
// and anything that isn't a real date is ignored. Read on every call, so a
// page left open overnight moves on to the next day.
const FAKE_TODAY = (() => {
  let fake = null;
  try { fake = new URLSearchParams(location.search).get("today"); } catch (e) { /* no query */ }
  return validDate(fake) ? fake : null;
})();
const today = () => FAKE_TODAY || isoDate(new Date());
const quizRoots = [...document.querySelectorAll("[data-quiz]")].filter(r => QUIZZES[r.dataset.quiz]);
const questionIds = new Map(); // "t3-4" -> number of options
quizRoots.forEach(root => {
  if (root.dataset.quiz === "exam") return;
  QUIZZES[root.dataset.quiz].questions.forEach((q, i) => questionIds.set(`${root.dataset.quiz}-${i + 1}`, q.options.length));
});
const questionEls = new Map(); // "t3-4" -> { fs, q, feedback }
function parseAnswers(raw) {
  const out = {};
  if (isPlainObject(raw)) {
    for (const [id, c] of Object.entries(raw)) {
      if (questionIds.has(id) && Number.isInteger(c) && c >= 0 && c < questionIds.get(id)) out[id] = c;
    }
  }
  return out;
}
function parseRetry(raw) {
  const out = {};
  if (isPlainObject(raw)) {
    for (const [id, d] of Object.entries(raw)) {
      // A date after today (a faked date, a wrong clock) would never come due: drop it.
      if (questionIds.has(id) && validDate(d) && d <= today()) out[id] = d;
    }
  }
  return out;
}
const answers = parseAnswers(readStored(QUIZ_KEY));
const retry = parseRetry(readStored(RETRY_KEY));
const isDue = id => Object.hasOwn(retry, id) && dayNumber(today()) - dayNumber(retry[id]) >= RETRY_AFTER_DAYS;
function replaceAll(target, fresh) {
  Object.keys(target).forEach(k => delete target[k]);
  Object.assign(target, fresh);
}
// Change one saved map. Start from what storage holds now, not the copy read
// at load, so a second open tab doesn't wipe this one's answers.
function updateStored(key, parse, target, change) {
  const raw = readStored(key);
  if (raw !== null) replaceAll(target, parse(raw));
  change(target);
  return writeStored(key, target);
}
// A question due for retry is asked cold: forget its saved answer.
// Returns the ids it cleared.
function askDueCold() {
  const cleared = Object.keys(retry).filter(id => isDue(id) && Object.hasOwn(answers, id));
  if (cleared.length) updateStored(QUIZ_KEY, parseAnswers, answers, a => cleared.forEach(id => delete a[id]));
  return cleared;
}
askDueCold();
// Show a question as saved: its checked answer and verdict, or blank.
function showSaved(qid) {
  const { fs, q, feedback } = questionEls.get(qid);
  const inputs = fs.querySelectorAll("input");
  inputs.forEach(r => { r.checked = false; });
  fs.querySelectorAll(".opt").forEach(l => l.classList.remove("is-ok", "is-bad"));
  feedback.className = "feedback";
  feedback.innerHTML = "";
  if (Object.hasOwn(answers, qid)) {
    inputs[answers[qid]].checked = true;
    showVerdict(fs, q, feedback, answers[qid]);
  }
}
function recordAnswer(qid, chosen, right) {
  if (!questionIds.has(qid)) return;
  // Only the first check of a question with no saved answer counts for the retry list.
  if (!Object.hasOwn(answers, qid)) {
    updateStored(RETRY_KEY, parseRetry, retry, r => { if (right) delete r[qid]; else r[qid] = today(); });
  }
  updateStored(QUIZ_KEY, parseAnswers, answers, a => { a[qid] = chosen; });
  document.dispatchEvent(new Event("rw-progress"));
}

document.querySelectorAll("[data-quiz]").forEach(root => {
  const id = root.dataset.quiz;
  const quiz = QUIZZES[id];
  if (!quiz) return;
  const body = root.querySelector(".quiz-body");
  const isExam = id === "exam";
  quiz.questions.forEach((q, i) => body.appendChild(renderQuestion(id, q, i, isExam)));
  if (quiz.tryit) {
    const p = document.createElement("p");
    p.className = "tryit";
    p.innerHTML = `<strong>Try it.</strong> ${quiz.tryit}`;
    body.appendChild(p);
  }
  if (isExam) {
    const bar = document.createElement("div");
    bar.className = "exam-bar";
    bar.innerHTML = `<button type="button" class="btn grade">Grade my exam</button>
      <button type="button" class="btn btn-quiet reset">Start over</button>`;
    body.appendChild(bar);
    const result = document.createElement("div");
    result.className = "exam-result";
    result.tabIndex = -1;
    result.setAttribute("aria-live", "polite");
    result.innerHTML = `<p class="muted">${lastExamText()}</p>`;
    body.appendChild(result);
    bar.querySelector(".grade").addEventListener("click", () => gradeExam(root));
    bar.querySelector(".reset").addEventListener("click", () => resetExam(root));
  }
});

/* Acceptance-criteria checkboxes remember themselves per browser. Stored
   data is untrusted: keep only ids that exist on this page. */
const CHECKS_KEY = "rust_at_work.checks";
const checkboxes = [...document.querySelectorAll("input.crit-box")];
const knownIds = new Set(checkboxes.map(b => b.id));
function parseChecks(raw) {
  return Array.isArray(raw) ? new Set(raw.filter(id => typeof id === "string" && knownIds.has(id))) : new Set();
}
function loadChecks() {
  try {
    return parseChecks(JSON.parse(localStorage.getItem(CHECKS_KEY) || "[]"));
  } catch (e) {
    return new Set();
  }
}
const checked = loadChecks();
checkboxes.forEach(box => {
  box.checked = checked.has(box.id);
  box.addEventListener("change", () => {
    if (box.checked) checked.add(box.id); else checked.delete(box.id);
    try { localStorage.setItem(CHECKS_KEY, JSON.stringify([...checked])); } catch (e) { /* storage blocked: still works for this visit */ }
  });
});
