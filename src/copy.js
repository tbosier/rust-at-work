/* Copy buttons for shell commands (pre.cmd) and review prompts (pre.prompt).
   Rust examples get none on purpose: the course wants them typed by hand.
   Where the Clipboard API is missing or refuses, as it can on file://, the
   button selects the block instead and says how to copy it. */
(() => {
  // A command block copies its commands only. If any line has a "$ " prompt,
  // the lines without one are output and stay behind.
  function copyText(pre) {
    const text = pre.textContent;
    if (!pre.classList.contains("cmd")) return text;
    const lines = text.split("\n");
    if (!lines.some(l => l.startsWith("$ "))) return text;
    return lines.filter(l => l.startsWith("$ ")).map(l => l.slice(2)).join("\n");
  }
  function commandCount(pre) {
    return pre.textContent.split("\n").filter(l => l.trim() && !l.trim().startsWith("#")).length;
  }
  function say(status, message) {
    // Clear first so the same message twice in a row is announced again.
    status.textContent = "";
    clearTimeout(status.timer);
    setTimeout(() => { status.textContent = message; }, 50);
    status.timer = setTimeout(() => { status.textContent = ""; }, 6000);
  }
  function selectBlock(pre) {
    const range = document.createRange();
    range.selectNodeContents(pre);
    const sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(range);
  }
  async function copy(pre, status) {
    try {
      if (!navigator.clipboard || !navigator.clipboard.writeText) throw new Error("no clipboard");
      await navigator.clipboard.writeText(copyText(pre));
      say(status, "Copied.");
    } catch (e) {
      selectBlock(pre);
      say(status, "Selected. Press Ctrl+C to copy.");
    }
  }
  document.querySelectorAll("pre.cmd, pre.prompt").forEach(pre => {
    const bar = document.createElement("div");
    bar.className = "copy-bar";
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "copy";
    btn.textContent = pre.classList.contains("cmd")
      ? (commandCount(pre) > 1 ? "Copy commands" : "Copy command")
      : "Copy prompt";
    const status = document.createElement("span");
    status.className = "copy-status";
    status.setAttribute("aria-live", "polite");
    btn.addEventListener("click", () => copy(pre, status));
    bar.append(status, btn);
    pre.classList.add("has-copy");
    pre.after(bar);
  });
})();
