# What's next

Part 1 (week zero and tickets 1 to 6) is done. Everything below is still to write or fix, roughly in the order
it's needed. Later parts get written once the learner finishes part 1, so the
reviews can decide what they push on.

## Part 2: pipeline

- [ ] An invoice simulator: a small program that keeps inserting new invoices
      (and the occasional duplicate or late correction) into a copy of
      `kettle_creek.db` while the learner's program runs.
- [ ] Ticket 7, "run it every 15 minutes". Pick up new invoices as they land,
      keep a high-water mark on disk, never count an invoice twice, and survive
      a restart halfway through. Teaches long-running programs, logging and
      state on disk. Under the hood: idempotency, clocks, at-least-once delivery.
- [ ] Ticket 8, "we have a million invoices now". Add a `--scale` option to
      `make_db.py`, then benchmark, profile, cut allocations and stream instead
      of loading everything. Under the hood: the memory hierarchy, cache
      locality, streaming versus batch.

## Part 3: service

- [ ] Ticket 9: an HTTP API in front of the reconciliation (axum, a first look at
      async and tokio, shared state). Under the hood: TCP and HTTP, concurrency
      versus parallelism.
- [ ] Ticket 10, "prove the API works": integration tests against a running
      server, error responses, an API contract, maybe property-based tests.

## Part 4: math in the service

- [ ] Ticket 11: a demand forecast endpoint. Least squares with the learner's own
      `Matrix` type first, then nalgebra. The order history in `po_lines` is the
      data. Under the hood: conditioning, and why the normal equations are the
      wrong way to compute least squares.
- [ ] Ticket 12, "which suppliers will bill late or wrong?". Logistic regression
      by hand with a train and test split. The database already has expected
      and received dates, and the generator gives each supplier its own
      chance of delivering late, so late rates are there to be found.
- [ ] Ticket 13, "the results weren't good enough". A stronger model, cross
      validation, leakage checks, and an honest comparison with the baseline.
- [ ] Capstone: a deliberately messy Rust service for the learner to review as
      the senior engineer.

## Other ideas from the original plan

- [ ] A math side track: dot product and norms, a `Matrix` struct, matrix
      multiplication, Gaussian elimination, LU and Cholesky. Small exercises
      that build toward ticket 11.
- [ ] A ticket on multiple currencies: a supplier that bills in CAD or EUR, and
      what that does to the `Cents` type.
- [ ] A journal template and a review log in the learner's project, so part 2
      can be written from what the reviews actually found.

## Known gaps in part 1

- [ ] Ticket 3's join timing (`snippets/out/t3_join_release.txt`) was captured
      by hand in a release build. `verify.py` doesn't regenerate it, so it can
      drift from the code.
- [ ] The hand-written file listings in tickets 4 and 5 are checked against
      the real files by a hidden example (`t4_zz_tests.rs`), not by the
      build. A general `{{file:path}}` placeholder in `build.py` would be
      simpler.
- [ ] A few sanity checks give away more than they should: ticket 2 shows one
      supplier's full Q3 row, and ticket 3's "under the hood" numbers imply the
      combined overbilled and underbilled count. Both are deliberate, and worth
      revisiting if they turn out to make the tickets too easy.
- [ ] The page assumes one learner's paths and reviewer. (Set up covers both
      Neovim and VS Code now.) A generic edition would make it easier for
      anyone else to pick up.
- [ ] Week zero's expected output lives on the page, not in `answer_key/`.
      An answer key entry for it would let reviews diff it like the tickets.
- [x] Another beginner read-through after the week zero rewrite (October 18,
      2026): a read-only audit of the whole page and a hands-on run of set up,
      week zero and tickets 1 and 2. Their findings are fixed.
- [ ] A hands-on beginner run of tickets 3 to 6. Only the read-only audit has
      covered them, and it named ticket 3 as the likeliest place to quit.
- [ ] The answer key sits in the same public repo as the course. Moving it to a
      private repo or a separate branch would make spoilers harder to stumble on.
- [ ] No CI yet. A GitHub Action that runs `verify.py`, `build.py` and the
      answer key's tests would catch a crate update that breaks an example.
