QUIZZES.t3 = {
  questions: [
    {
      q: "Why does <code>0.10 + 0.20 == 0.30</code> come out <code>false</code> in <code>f64</code>?",
      options: [
        { t: "Neither 0.1 nor 0.2 can be stored exactly in binary, and their rounding errors don't cancel when added", ok: true },
        { t: "<code>f64</code> only keeps about seven significant digits", why: "That's <code>f32</code>. <code>f64</code> keeps 15 to 17, and the stray digit here is in the seventeenth place. More digits make the error smaller, not zero." },
        { t: "Rust adds floats in a different order from Python", why: "Python gives exactly the same answer, <code>0.30000000000000004</code>. Both use IEEE 754 binary64 and the hardware's addition." },
      ],
      explain: "1/10 repeats forever in base 2, like 1/3 in base 10. Each stored value is the nearest one available, and the sum of two nearest values isn't always the nearest value to the sum.",
    },
    {
      q: "You declare <code>enum Kind { OrphanInvoice, DuplicateInvoice, ..., NotInvoiced }</code> with <code>#[derive(PartialOrd, Ord, ...)]</code>. How do kinds sort?",
      options: [
        { t: "Alphabetically by variant name", why: "Then <code>BilledCancelled</code> would come first. The derived order knows nothing about names." },
        { t: "In the order the variants are declared", ok: true },
        { t: "They can't be sorted until you write <code>cmp</code> by hand", why: "That's what the derive writes for you." },
      ],
      explain: "Declare the variants in report order, derive <code>Ord</code>, and sorting by kind is correct by construction. No separate list of strings to keep in step.",
    },
    {
      q: "The <code>match</code> that decides a PO's kind ends with <code>_ =&gt; None</code>. Next month someone adds <code>Status::OnHold</code>. What happens?",
      options: [
        { t: "Every on-hold PO silently lands in the <code>_</code> arm and never appears on the report", ok: true },
        { t: "The compiler reports a non-exhaustive match", why: "It would without the <code>_</code> arm. The catch-all is exactly what tells the compiler every case is handled." },
        { t: "The program panics on the first on-hold PO", why: "Nothing panics. The wildcard matches and returns <code>None</code>. That's what makes it dangerous." },
      ],
      explain: "An exhaustive <code>match</code> with no wildcard turns \"we forgot a case\" into a compile error. In code that decides what to report, that's the check you want.",
    },
    {
      q: "<code>fn load_catalog&lt;'a&gt;() -&gt; Catalog&lt;'a&gt;</code> builds a <code>Vec</code> of books inside itself and returns a catalog that borrows from it. Why won't it compile?",
      code: "fn load_catalog<'a>() -> Catalog<'a> {\n    let books = vec![/* ... */];\n    Catalog::new(&books)\n}",
      options: [
        { t: "<code>books</code> is freed when the function returns, and the catalog would still point at it", ok: true },
        { t: "Structs can't hold references", why: "They can, with a lifetime parameter, as <code>Catalog&lt;'a&gt;</code> does. The problem is what this one would point at." },
        { t: "The lifetime is spelled wrong; it should be <code>'static</code>", why: "<code>'static</code> would promise the books live for the whole program, which they don't either. A lifetime describes what the code does; it can't change it." },
      ],
      explain: "Own the data higher up (load the <code>Vec</code> in the caller and pass <code>&amp;books</code> in), or let the index own its data outright.",
    },
    {
      q: "A nested-loop join of 2,000 invoices against 2,000 POs takes 0.4 ms. Both tables grow to 8,000 rows. Roughly how long now?",
      options: [
        { t: "About 1.6 ms", why: "That's linear growth, the hash join's pattern. The nested loop does <em>n</em> &times; <em>m</em> work, and both went up 4 times." },
        { t: "About 6 ms", ok: true },
        { t: "About 0.4 ms; computers are fast", why: "Fast enough today. The measured table shows the same code at 1.7 seconds with 64 times the data." },
      ],
      explain: "4 times as many invoices, each searching 4 times as many POs: about 16 times the work. The measured run went from 0.4 ms to 7.1 ms.",
    },
    {
      q: "Why make <code>struct Cents(i64)</code> rather than using a plain <code>i64</code> for money?",
      options: [
        { t: "A newtype is faster than an <code>i64</code>", why: "It's exactly as fast. A one-field struct compiles to the same <code>i64</code>." },
        { t: "So the compiler stops money mixing with quantities and IDs, and so money gets its own <code>Display</code>", ok: true },
        { t: "Because an <code>i64</code> can't hold large enough amounts", why: "An <code>i64</code> holds about 9.2 &times; 10<sup>18</sup> cents. Size isn't the issue." },
      ],
      explain: "With plain <code>i64</code>, <code>price + po_id</code> compiles. With <code>Cents</code>, it doesn't. Zero cost at run time, and a whole class of mistakes gone at compile time.",
    },
  ],
  tryit: "Add a variant <code>Damaged</code> to the library's <code>Standing</code> enum and build. Read every error: each one is a place the new case needs a decision. Then add <code>_ =&gt; String::new()</code> to <code>action</code> and build again. The compiler goes quiet, and that's the problem.",
};
