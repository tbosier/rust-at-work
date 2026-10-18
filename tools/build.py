"""Assemble dist/rust_at_work.html from src/ and the verified snippets.

    python3 tools/verify.py && python3 tools/build.py

Placeholders in src/*.html:
    {{include:sections/t1.html}}   another source file, itself expanded
    {{code:t1_load#all}}           snippets/examples/t1_load.rs, region "all"
    {{code:lib#library}}           snippets/src/lib.rs, region "library"
    {{out:t1_load}}                captured stdout of that example
    {{fail:t1_move}}               snippets/fail/t1_move.rs
    {{failout:t1_move}}            the compiler's message for it
    {{quizzes}}                    every src/quizzes/*.js, in name order

The build refuses to write if a placeholder is unknown, a region is missing,
an internal link has no target, an id repeats, or the text has dashes the
house style bans.
"""
import html
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
SNIP = ROOT / "snippets"
errors = []


def region(path, name):
    """The lines of a named region, or the whole file when name is None.

    A region name may appear more than once in a file, for example around
    the `use` lines at the top and again inside main. The pieces are shown
    in file order with a blank line between them.
    """
    lines = path.read_text().splitlines()
    if name is None:
        pieces = [lines]
    else:
        pieces = []
        i = 0
        while i < len(lines):
            if lines[i].strip() == f"// region: {name}":
                end = next((j for j in range(i + 1, len(lines)) if lines[j].strip() == "// endregion"), None)
                if end is None:
                    errors.append(f"unclosed region {name!r} in {path.name}")
                    return ""
                pieces.append(lines[i + 1:end])
                i = end
            i += 1
        if not pieces:
            errors.append(f"no region {name!r} in {path.name}")
            return ""
    out = []
    for body in pieces:
        body = [l for l in body if not re.match(r"\s*// (region: .*|endregion)$", l)]
        # Remove the common indent so a region inside main() starts at column 0.
        indents = [len(l) - len(l.lstrip()) for l in body if l.strip()]
        cut = min(indents) if indents else 0
        out.append("\n".join(l[cut:] for l in body).strip("\n"))
    return "\n\n".join(out)


def read(path):
    if not path.exists():
        errors.append(f"missing file {path.relative_to(ROOT)}")
        return ""
    return path.read_text()


def expand(text):
    def sub(m):
        kind, arg = m.group(1), m.group(2)
        if kind == "include":
            return expand(read(SRC / arg))
        if kind == "quizzes":
            return "\n".join(p.read_text() for p in sorted((SRC / "quizzes").glob("*.js")))
        name, _, reg = (arg or "").partition("#")
        if kind == "code":
            path = SNIP / "src" / "lib.rs" if name == "lib" else SNIP / "examples" / f"{name}.rs"
            if not path.exists():
                errors.append(f"missing snippet {name}")
                return ""
            return html.escape(region(path, reg or None), quote=False)
        if kind == "fail":
            path = SNIP / "fail" / f"{name}.rs"
            if not path.exists():
                errors.append(f"missing failing snippet {name}")
                return ""
            return html.escape(region(path, None), quote=False)
        if kind == "out":
            text = read(SNIP / "out" / f"{name}.txt")
            if reg:
                parts = re.split(r"(?m)^---\n", text)
                if not reg.isdigit() or not 1 <= int(reg) <= len(parts):
                    errors.append(f"no part {reg} in output of {name}")
                    return ""
                text = parts[int(reg) - 1]
            return html.escape(text.rstrip("\n"), quote=False)
        if kind == "failout":
            return html.escape(read(SNIP / "out" / f"fail_{name}.txt").rstrip("\n"), quote=False)
        errors.append(f"unknown placeholder {m.group(0)}")
        return ""

    return re.sub(r"\{\{(include|code|out|failout|fail|quizzes)(?::([^}]*))?\}\}", sub, text)


def check(page):
    for m in re.finditer(r"\{\{[^}]*\}\}", page):
        errors.append(f"unexpanded {m.group(0)}")
    ids = re.findall(r'\sid="([^"]+)"', page)
    seen = set()
    for i in ids:
        if i in seen:
            errors.append(f"duplicate id {i}")
        seen.add(i)
    for target in set(re.findall(r'href="#([^"$]+)"', page)):
        if target not in seen:
            errors.append(f"link to missing #{target}")
    for ch, name in (("—", "em dash"), ("–", "en dash")):
        n = page.count(ch)
        if n:
            errors.append(f"{n} {name}(s)")
    for word in (r"\bgrey\b", "colour", "behaviour", "organise", "realise", "analyse", "centre", "favour", "licence"):
        if re.search(word, page, re.I):
            errors.append(f"British spelling: {word}")


def main():
    page = expand((SRC / "guide.html").read_text())
    check(page)
    if errors:
        print("\n".join(errors))
        sys.exit(f"{len(errors)} problem(s); nothing written")
    out = ROOT / "dist" / "rust_at_work.html"
    out.parent.mkdir(exist_ok=True)
    out.write_text(page)
    print(f"wrote {out} ({len(page) // 1024} KB)")


if __name__ == "__main__":
    main()
