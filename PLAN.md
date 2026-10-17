# Rust at work: authoring contract and ticket specs

The deliverable is one HTML file, `dist/rust_at_work.html`, copied to
`~/projects/space_sim/learning/rust_at_work.html`. The database
`data/kettle_creek.db` (built by `data/make_db.py`) is copied to
`~/projects/space_sim/learning/data/`. The learner (comfortable with
Python and advanced math, new-ish to Rust) codes in their own project
`~/projects/kettle_creek` in Neovim. Claude reviews; it never writes their code.

## Build and verify

    python3 tools/verify.py     # compiles + runs snippets/examples, compiles snippets/fail (must fail)
    python3 tools/build.py      # assembles src/guide.html -> dist/, refuses on errors

- Every code block on the page comes from a placeholder, never typed by hand:
  `{{code:NAME#region}}` (snippets/examples/NAME.rs, `// region: x` ... `// endregion`),
  `{{code:lib#library}}`, `{{out:NAME}}` or `{{out:NAME#k}}` (k-th part of stdout
  split on a line `---`), `{{fail:NAME}}` + `{{failout:NAME}}` (snippets/fail/NAME.rs
  and the real compiler message). Hand-written `<pre>` only for shell commands,
  Cargo.toml fragments, prompts and data excerpts.
- Name files by ticket: `t3_*.rs`, `drill_t3.rs` (hidden program that produces drill
  outputs from the real DB at `../../data/kettle_creek.db`; cwd is snippets/out).
- Examples must pass `cargo clippy --examples -- -D warnings`.
- Every claim about behavior, timing or output must come from something that ran.

## Page conventions

- Each ticket section `src/sections/tN.html`, in this order:
  h2#tN, email (`article.email`), `p.teaches`, `p.wiki-crumb#tN-wiki` + h3 wiki title,
  wiki subsections (h4), h3 Warm-up drills (`div.drill`), h3 The ticket
  (`ul.criteria` with checkbox ids `c-tN-k`, then a "Before you type" paragraph),
  h3 Stuck? (`details.hint`, 3 rungs, each a nudge not a solution), h3 Sanity checks,
  h3 Hand it in (ticket-specific review focus in `pre.prompt`, plus "reply to Dana"),
  h3 Under the hood (CS, h4 subsections), h3 Go deeper (links list),
  quiz section `data-quiz="tN"`.
- Quizzes: `src/quizzes/tN.js` assigns `QUIZZES.tN = { questions: [...], tryit: "..." }`.
  Each question `{ q, code?, options: [{ t, ok?, why }], explain }`. 4 to 6 questions.
  Wrong options carry a `why` that teaches. Exam lives in `src/quizzes/zz_exam.js`.
- Wiki examples use the in-memory lending library (`snippets::library()`: books,
  loans, fines with REAL amounts) or tiny inline data. NEVER the ticket's own tables,
  so nothing can be copied into the answer. Drills use the real DB but different
  tables or questions from the ticket.
- Python comparisons where they help (the learner knows Python well).
- Voice: plain, direct, second person, American English, no em or en dashes
  (build refuses them), no Title Case headings, no marketing words, no reflexive
  triplets. Explain why, not just how. Short paragraphs.
- Inline code needs `&lt;` `&gt;` `&amp;` escaped by hand.
- Diagrams: inline SVG using CSS vars (--sea, --sea-edge, --depth, --sheet, --rule,
  class "tick" for labels), wrapped in `div.diagram role="img" aria-label=...`.

## Concept allocation (what each ticket teaches; don't re-teach, do build on)

| T | Rust | CS under the hood |
|---|------|-------------------|
| 1 | cargo, struct, String vs &str (light), Option for NULL, rusqlite prepare/query_map/get/collect, Result + ?, Box<dyn Error> main, Vec, iter/filter/count, closures, borrowing &[T], moves, a first lifetime, sort_by_key/sort_by stable, tuple keys, BufWriter + writeln!, const | stack vs heap, Vec ptr/len/cap, amortized growth, niche, lexicographic order of ISO dates |
| 2 | HashMap (entry API, or_insert/or_default, get -> Option), &str keys borrowed from a Vec, String vs &str properly, clone cost, if let / match on Option, expect with a message, sorting with Ordering::then_with on totals, f64 formatting {:.2}, csv crate Writer (write_record, serialize w/ serde not used), HashMap iteration order | hash tables (hashing, buckets, collisions, load factor, O(1) average), SipHash + random seed (HashDoS) => why iteration order changes run to run, comparison sort O(n log n), stability |
| 3 | enums with data, exhaustive match, derive Debug/Clone/Copy/PartialEq/Eq/PartialOrd/Ord/Hash, a Cents newtype with Display and Add/AddAssign/Sum, HashSet, Option combinators (map, copied, unwrap_or, unwrap_or_default), let-else, lifetimes properly (struct holding &'a, borrow checker error when the owner is dropped), matches!, sorting by derived Ord on an enum | IEEE 754 binary64 (0.1 not representable, why == fails, rounding once at the boundary, Goldberg), big-O, nested-loop join vs hash join with a measured timing in --release, what "index" means |
| 4 | custom error enum, impl Display + std::error::Error, From + ? conversion, parsing text (trim, strip_prefix, split_once, bytes().all(is_ascii_digit), parse::<i64>), never panicking on input, #[cfg(test)] mod tests, #[test], assert_eq!, tests/ integration tests, lib.rs + main.rs, mod/pub/use crate::, visibility | parsing and grammars (formats are tiny languages), "parse, don't validate" (Alexis King), Gregorian leap rule, testing theory (boundaries, equivalence classes, tests show presence not absence of bugs, Dijkstra) |
| 5 | traits (define + impl for several types), generics with trait bounds, static dispatch, clap derive CLI, rust_xlsxwriter, errors to stderr + exit codes, validating CLI input (--month YYYY-MM) | interfaces and dispatch (monomorphization vs vtables / dyn), xlsx = zip of XML (they can unzip it), processes: stdout vs stderr, exit status, why automation cares (sets up Part 2) |

## The story and dates

- Company: Kettle Creek Supply, industrial parts distributor. Sim "today" = Mon 2026-10-05.
- Dana Whitlock, Operations Manager (boss). Priya Raman, Accounts Payable lead (from T2).
- Avoid pronouns for both (use names).
- T1 Mon Oct 5 8:12 AM. T2 Wed Oct 7. T3 Mon Oct 12. T4 Thu Oct 15. T5 Mon Oct 19.
- Learner's project: ~/projects/kettle_creek, DB at data/kettle_creek.db, outputs in out/.
- House rule: SQL is only `SELECT columns FROM table`. All logic in Rust.

## Ticket specs (the answer key implements exactly these)

Answer key: `answer_key/` (`cargo run -- tN`), outputs in `answer_key/out/tN/`.
Sanity checks may reveal loading facts and partial counts, not the answers.

### T1 open POs
Count status == "open" (70). Stale = open and order_date < "2026-08-06" (18).
`out/stale_open_pos.csv`: `po_id,supplier_id,buyer,order_date`, sort order_date then po_id.

### T2 spend by supplier, Q3 2026
All rows of `invoices` with invoice_date in 2026-07-01..=2026-09-30 (string compare).
`out/spend_by_supplier_2026Q3.csv`: `supplier_id,supplier,invoices,total`; total with
two decimals, no $; highest total first, ties by supplier name; only suppliers with
>= 1 invoice. Print a grand total line. Answer: 21 suppliers, 268 invoices,
1946555.14. Learner uses f64 here (they don't know better yet); `{:.2}` makes it
right; raw `{}` printing shows noise like 85542.55000000002 for 15 of 21 suppliers.
T2 can show that noise as a warning sign; the float trap really bites in T3, where
amounts are compared for equality.
Allowed crates: rusqlite, csv.

### T3 reconcile (MAIN branch only)
- Scope: POs with branch "MAIN". Millbrook POs are out of scope (their invoices are
  in another system until T4). Invoices whose po_id is a Millbrook PO: none exist.
- expected(PO) = sum over po_lines of quantity * unit_price, in integer cents,
  unit price cents = round(unit_price * 100).
- Invoices in invoice_id order. If an invoice's po_id matches no PO: ORPHAN_INVOICE
  (expected blank, billed = amount, difference blank). If (po_id, invoice_number)
  already seen: DUPLICATE_INVOICE (same blanks), not added to billed.
- Per MAIN PO: billed = sum of non-duplicate invoices.
  cancelled + has invoices -> BILLED_CANCELLED (expected 0.00);
  open + has invoices -> BILLED_OPEN (0 rows in this data; must still be handled);
  received + no invoices -> NOT_INVOICED only if received_date <= "2026-09-04";
  received + billed > expected -> OVERBILLED; < -> UNDERBILLED; equal -> fine.
- `out/discrepancies.csv`: `kind,po_id,invoice_number,supplier,expected,billed,difference`
  (difference = billed - expected; supplier name; invoice_number only for ORPHAN and
  DUPLICATE). Sorted by kind in the order ORPHAN_INVOICE, DUPLICATE_INVOICE,
  BILLED_CANCELLED, BILLED_OPEN, OVERBILLED, UNDERBILLED, NOT_INVOICED, then po_id,
  then invoice_number. Print count per kind and total overbilled.
- Answer: ORPHAN 4, DUPLICATE 22, BILLED_CANCELLED 3, BILLED_OPEN 0, OVERBILLED 73,
  UNDERBILLED 34, NOT_INVOICED 32; total overbilled 8607.85; 168 rows.
- With f64 and == the learner would see many false mismatches: measure how many
  with a hidden program and use the real number in the guide.
- Allowed crates: rusqlite, csv.

### T4 Millbrook branch invoices
Table branch_invoices (all TEXT: ref, vendor, po_ref, inv_date, amount, entered_by).
Check fields in this order; the first failure rejects the row:
Rows are processed in rowid order (the order they were keyed; the learner may
`SELECT rowid, ...`). rejects.csv is in that order. Reason text is free; reviews
compare rejects by ref and field.
1. ref: trimmed, non-empty ("missing"); seen before -> "duplicate ref" (any earlier
   row, by rowid, with the same ref, accepted or not).
2. vendor: normalize (lowercase, split_whitespace joined by single spaces,
   " and " -> " & ", drop one trailing "."), compare with normalized supplier names;
   no match -> "unknown vendor". Truncated names like "Old Varrow" don't match.
3. po_ref: trim; optional prefix "PO-" or "PO " (case-sensitive, as in data);
   rest all ASCII digits and non-empty, else "not a PO number"; PO must exist
   ("unknown PO") and belong to the vendor's supplier ("PO belongs to another supplier").
4. inv_date: trim; empty -> "missing"; YYYY-MM-DD (exactly 4-2-2 digits) or
   M/D/YYYY (1-2, 1-2, 4 digits, US order); else "not YYYY-MM-DD or MM/DD/YYYY";
   must be a real date incl. leap years -> "no such date".
5. amount: trim; empty -> "missing"; "(x)" or leading "-" = negative (credit);
   optional "$"; commas removed; digits with optional "." and 0-2 decimals
   ("more than two decimal places" if 3+); anything else "not a number".
   Parse straight to cents, never via f64.
`out/rejects.csv`: `ref,field,value,reason` (value = raw text).
Then reconcile with Millbrook included (accepted branch rows add to billed for their
PO; duplicates don't apply to branch rows beyond the duplicate-ref rule).
`out/discrepancies.csv` again, same format, now both branches.
Answer: accepted 143, rejected 40 (amount: missing 2, 3+ decimals 7, not a number 4;
inv_date: missing 3, no such date 6; po_ref: not a PO number 13; ref: duplicate 1;
vendor: unknown 4). Reconcile: ORPHAN 4, DUP 22, BILLED_CANCELLED 3, BILLED_OPEN 0,
OVERBILLED 80, UNDERBILLED 37, NOT_INVOICED 69; total overbilled 9084.19; 215 rows.
Also: refactor into lib.rs + modules, an error enum, unit tests for every parser,
an integration test in tests/. Allowed crates: rusqlite, csv.

### T5 monthly pack
`cargo run -- --month 2026-09` (clap derive; also `--db` default data/kettle_creek.db,
`--out` default out/kettle_pack_<month>.xlsx). Bad month -> message on stderr,
non-zero exit, no file.
Workbook (rust_xlsxwriter), sheets in order:
1. Summary: title, month, invoices dated this month (main non-duplicate + accepted
   Millbrook), spend this month, open discrepancy counts by kind (the full
   reconciliation as of today, not month-filtered), Millbrook rows rejected.
2. Spend: Supplier, Invoices, Total for invoices dated in the month (main invoices
   excluding duplicate copies; plus accepted Millbrook rows), sorted by total desc
   then name. Money as numbers with a currency format, not text.
3. Discrepancies: the full T4 reconciliation (Kind, PO, Invoice number, Supplier,
   Expected, Billed, Difference).
4. Rejects: Ref, Field, Value, Reason.
Every table sheet: bold header, frozen header row, autofilter, sensible widths.
Answer for 2026-09: 21 suppliers, 98 invoices, spend 620883.66.
Traits: one `SheetRow`-style trait implemented by each row type, one generic
`write_table<T: ...>` used for all three table sheets.
Allowed crates: rusqlite, csv, rust_xlsxwriter, clap.
