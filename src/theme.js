/* The theme switch: System, Light or Dark. This runs in <head>, so a saved
   choice applies before the first paint and the page never flashes the
   wrong colors. Stored data is untrusted: anything but the three names
   means System. Without storage the switch still works for this visit. */
(() => {
  const KEY = "rust_at_work.theme";
  const CHOICES = ["system", "light", "dark"];
  const root = document.documentElement;
  const normalize = v => (CHOICES.includes(v) ? v : "system");
  function load() {
    try { return normalize(localStorage.getItem(KEY)); } catch (e) { return "system"; }
  }
  function apply(choice) {
    if (choice === "system") delete root.dataset.theme;
    else root.dataset.theme = choice;
  }
  apply(load());
  document.addEventListener("DOMContentLoaded", () => {
    const box = document.querySelector("fieldset.theme");
    if (!box) return;
    const current = load();
    box.querySelectorAll('input[name="theme"]').forEach(radio => {
      radio.checked = radio.value === current;
      radio.addEventListener("change", () => {
        if (!radio.checked) return;
        const choice = normalize(radio.value);
        apply(choice);
        try { localStorage.setItem(KEY, choice); } catch (e) { /* storage blocked */ }
      });
    });
    box.hidden = false;
  });
})();
