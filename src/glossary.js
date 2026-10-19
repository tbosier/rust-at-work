/* Glossary popovers. Without JavaScript a term link (a.term) just jumps to
   its entry in #glossary. With it, activating the link opens a small
   non-modal dialog beside the word: the definition copied from the entry,
   a link to where the page teaches it, and a link to the entry itself.
   Escape, Close or a click elsewhere shuts it, and focus goes back to the
   term. One dialog is reused, moved in the DOM to sit right after the open
   term, so Tab carries on from there. */
(() => {
  const terms = [...document.querySelectorAll("a.term")];
  if (!terms.length) return;
  const GAP = 6, MARGIN = 8;

  const pop = document.createElement("div");
  pop.className = "g-pop";
  pop.id = "g-pop";
  pop.setAttribute("role", "dialog");
  pop.setAttribute("aria-labelledby", "g-pop-title");
  pop.setAttribute("aria-describedby", "g-pop-def");
  pop.tabIndex = -1;
  pop.hidden = true;
  pop.innerHTML = `<p class="g-pop-title" id="g-pop-title"></p>
    <div class="g-pop-def" id="g-pop-def"></div>
    <p class="g-pop-links"><a class="g-pop-taught" href="#glossary"></a>
      <a class="g-pop-entry" href="#glossary">Glossary</a>
      <button type="button" class="linkbtn g-pop-close">Close</button></p>`;
  const title = pop.querySelector(".g-pop-title");
  const def = pop.querySelector(".g-pop-def");
  const taught = pop.querySelector(".g-pop-taught");
  const entry = pop.querySelector(".g-pop-entry");
  let current = null;

  terms.forEach(a => {
    a.setAttribute("aria-haspopup", "dialog");
    a.setAttribute("aria-expanded", "false");
  });

  // The entry a term points at, or null. Only ids shaped like g-word count.
  function entryFor(a) {
    const id = (a.getAttribute("href") || "").slice(1);
    if (!/^g-[a-z0-9-]+$/.test(id)) return null;
    const dt = document.getElementById(id);
    const dd = dt && dt.nextElementSibling;
    return dt && dt.tagName === "DT" && dd && dd.tagName === "DD" ? { id, dt, dd } : null;
  }

  function place(a) {
    const rects = a.getClientRects();
    const r = rects.length ? rects[rects.length - 1] : a.getBoundingClientRect();
    const vw = document.documentElement.clientWidth, vh = window.innerHeight;
    const w = pop.offsetWidth, h = pop.offsetHeight;
    const left = Math.max(MARGIN, Math.min(r.left, vw - w - MARGIN));
    // Below the word if it fits, else above it, else as low as it can go
    // while staying on screen. CSS caps its height at the viewport's.
    let top = r.bottom + GAP;
    if (top + h > vh - MARGIN) {
      top = r.top - GAP - h >= MARGIN ? r.top - GAP - h : Math.max(MARGIN, vh - MARGIN - h);
    }
    pop.style.left = `${left + window.scrollX}px`;
    pop.style.top = `${top + window.scrollY}px`;
  }

  function open(a) {
    const e = entryFor(a);
    if (!e) return false;
    if (current) current.setAttribute("aria-expanded", "false");
    title.textContent = e.dt.textContent.trim();
    def.replaceChildren(...[...e.dd.querySelectorAll(".g-def")].map(p => p.cloneNode(true)));
    const where = e.dd.querySelector("a.g-taught");
    taught.hidden = !where;
    if (where) {
      taught.href = where.getAttribute("href");
      taught.textContent = `Where it's taught: ${where.textContent.trim()}`;
    }
    entry.href = `#${e.id}`;
    a.after(pop);
    a.setAttribute("aria-controls", "g-pop");
    a.setAttribute("aria-expanded", "true");
    current = a;
    pop.hidden = false;
    place(a);
    pop.focus({ preventScroll: true });
    return true;
  }

  function close(returnFocus) {
    if (!current) return;
    const a = current;
    current = null;
    pop.hidden = true;
    a.setAttribute("aria-expanded", "false");
    if (returnFocus) a.focus({ preventScroll: true });
  }

  document.addEventListener("click", ev => {
    const a = ev.target.closest && ev.target.closest("a.term");
    if (a) {
      // Ctrl, Cmd or Shift-click still opens the entry the browser's way.
      if (ev.ctrlKey || ev.metaKey || ev.shiftKey || ev.altKey || ev.button !== 0) return;
      if (a === current) { ev.preventDefault(); close(true); return; }
      if (open(a)) ev.preventDefault();
      return;
    }
    if (!current || pop.contains(ev.target)) return;
    // A click on plain page text leaves focus nowhere useful, so send it
    // back to the term. A click on something focusable keeps its focus.
    const lost = document.activeElement === document.body || pop.contains(document.activeElement);
    close(lost);
  });

  pop.querySelector(".g-pop-close").addEventListener("click", () => close(true));

  // Following either link closes the dialog and moves focus to the place
  // it jumps to, so keyboard users land where they see.
  [taught, entry].forEach(link => link.addEventListener("click", () => {
    const target = document.getElementById(link.getAttribute("href").slice(1));
    close(false);
    if (target) {
      if (!target.hasAttribute("tabindex")) target.setAttribute("tabindex", "-1");
      requestAnimationFrame(() => target.focus({ preventScroll: true }));
    }
  }));

  document.addEventListener("keydown", ev => {
    if (ev.key === "Escape" && current) {
      ev.preventDefault();
      close(true);
    }
  });

  // Focus moving anywhere but the dialog or its own term closes it, so
  // Tab or Shift+Tab past the term never leaves a stale dialog open.
  document.addEventListener("focusin", ev => {
    if (current && !pop.contains(ev.target) && ev.target !== current) close(false);
  });

  window.addEventListener("resize", () => { if (current) place(current); });
})();
