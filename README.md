# Rust at work

A Rust course built as a simulated job. You join Kettle Creek Supply, a made-up
distributor of industrial parts. Your manager emails you tickets, and each one
needs a little more Rust than the last. Under every email is a short wiki page
that teaches just enough for that ticket, some warm-up drills, a checklist,
hints you open one at a time, and a section on the computer science underneath.
You write the code yourself. When a ticket is done, you ask Claude for a review
the way a senior engineer would review a pull request, and it doesn't rewrite
your code.

It's written for someone who already gets work done in Python, probably in
notebooks, but has never studied computer science. Nothing is assumed: a
"week zero" section covers the basics (compiling, types, loops, structs,
`Option` and `Result`, reading the docs) and what's underneath them (bits,
memory, text encoding, processes) before the first ticket arrives.

Part 1 (week zero and six tickets) is written. It goes from counting rows in a
SQLite database to a reconciliation of invoices against purchase orders, a
parser for a messy acquired branch's data, a refactor into a tested library
with modules, and a monthly Excel workbook with a command line. Parts 2 to 4
(a scheduled pipeline, an HTTP API, and linear algebra and machine learning
inside that API) are planned. See [WHATS_NEXT.md](WHATS_NEXT.md).

## Start here

1. Open `dist/rust_at_work.html` in a browser. It's one self-contained file.
2. Follow "Set up". It makes your own cargo project and copies in the
   practice database, `data/kettle_creek.db`. Then do week zero.

The page was written for one learner, so a few things assume that setup:

- Paths like `~/projects/kettle_creek`, `~/projects/space_sim/learning/` and
  `~/projects/testRandomShiz/rust_at_work/`. They appear in the setup commands,
  in the `CLAUDE.md` you paste on day one and in the review prompt. Change all
  three to match where you put things.
- A Neovim config with rustaceanvim. The keys it lists come from that config.
  The set up section also has the VS Code equivalents.
- Claude Code as the reviewer, with a `CLAUDE.md` that stops it from writing
  your code. Any reviewer who follows the review checklist on the page will do.

You need Rust 1.89 or newer and an internet connection for the first build.

## Spoilers

`answer_key/` is a complete solution to every ticket, and `answer_key/out/`
holds its output files. Reviews compare your output against them. If you're
taking the course, don't open that folder.

## What's in the repo

| Path | What it is |
|---|---|
| `dist/rust_at_work.html` | The built guide |
| `data/make_db.py` | Builds `data/kettle_creek.db`, the synthetic database. Deterministic: same seed, same rows |
| `src/` | Sources for the guide: one HTML file per section, one quiz file per ticket |
| `snippets/` | Every code block on the page. `examples/` must compile and run, `fail/` must fail to compile, and `out/` holds what they printed |
| `answer_key/` | The reference solution (see above) |
| `tools/` | `verify.py`, `build.py`, and two browser checks |
| `PLAN.md` | The authoring rules and the exact spec for each ticket |

## Building the guide

```sh
python3 tools/verify.py   # compile and run every snippet, capture outputs
python3 tools/build.py    # assemble dist/rust_at_work.html
```

`build.py` refuses to write the page if a snippet is missing, an internal link
points nowhere, an id repeats, or the text has an em dash or a British spelling.
Code blocks come from files that were compiled, and each output is what that
file printed. The exceptions are shell commands and three file listings in
tickets 4 and 5, which `verify.py` checks against the real files.

The browser checks (`tools/shot.mjs` and `tools/enginetest.mjs`) need
`playwright-core` and a Chrome binary. Set `PLAYWRIGHT_FROM` to a
`package.json` whose `node_modules` has playwright-core, and `CHROME` if Chrome
isn't at `/usr/bin/google-chrome-stable`.

To regenerate the answer key's outputs: `answer_key/run_all.sh`.

## License

MIT. Kettle Creek Supply, its suppliers, its staff and all of its data are
fictional. Any resemblance to a real business or person is a coincidence.
