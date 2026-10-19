"""Build-time coloring for Rust code and for compiler output.

    rust(code)      -> HTML for raw Rust source, with <span class="sx-..">
    compiler(text)  -> HTML for raw rustc/cargo output, with <span class="cc-..">

Both take raw text (not yet escaped) and escape every piece themselves, so
stripping the tags and unescaping gives back exactly the input. The page
stays copyable: the spans only add color.

python3 tools/test_highlight.py runs the tricky cases.
"""
import html
import re

KEYWORDS = set("""
    as async await break const continue crate dyn else enum extern false fn for
    if impl in let loop match mod move mut pub ref return self Self static struct
    super trait true type unsafe use where while
""".split())
PRIMITIVES = set("""
    bool char str u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64
""".split())

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
NUMBER = re.compile(
    r"0x[0-9a-fA-F_]+|0o[0-7_]+|0b[01_]+"
    r"|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?"
    r"(?:_?(?:u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize|f32|f64))?"
)
# A char literal: one character or one escape between quotes. Tried before a
# lifetime, so 'a' is a char and the 'a in &'a str is a lifetime.
CHAR = re.compile(r"b?'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]{1,6}\}|.)|[^\\'\n])'")
LIFETIME = re.compile(r"'[A-Za-z_][A-Za-z0-9_]*")
RAW_STR = re.compile(r'b?r(#*)"')
STR = re.compile(r'b?"(?:\\.|[^"\\])*"', re.S)


def _span(cls, text):
    return f'<span class="{cls}">{html.escape(text, quote=False)}</span>'


def _block_comment_end(code, i):
    """Index just past the */ that closes the comment opened at i. Rust block
    comments nest."""
    depth, j = 0, i
    while j < len(code):
        if code.startswith("/*", j):
            depth, j = depth + 1, j + 2
        elif code.startswith("*/", j):
            depth, j = depth - 1, j + 2
            if depth == 0:
                return j
        else:
            j += 1
    return len(code)


def _attribute_end(code, i):
    """Index just past the ] that closes #[ or #![ at i, skipping strings."""
    j = code.index("[", i) + 1
    depth = 1
    while j < len(code) and depth:
        m = STR.match(code, j)
        if m:
            j = m.end()
            continue
        if code[j] == "[":
            depth += 1
        elif code[j] == "]":
            depth -= 1
        j += 1
    return j


def tokens(code):
    """Yield (class or None, text) pairs that join back into code."""
    i, n = 0, len(code)
    plain = []

    def word_start(k):
        return k == 0 or not (code[k - 1].isalnum() or code[k - 1] == "_")

    def flush():
        if plain:
            yield None, "".join(plain)
            plain.clear()

    while i < n:
        c = code[i]
        hit = None
        if code.startswith("//", i):
            end = code.find("\n", i)
            end = n if end < 0 else end
            hit = ("sx-com", end)
        elif code.startswith("/*", i):
            hit = ("sx-com", _block_comment_end(code, i))
        elif c == "#" and re.match(r"#!?\[", code[i:i + 3]):
            hit = ("sx-attr", _attribute_end(code, i))
        elif c == '"':
            hit = ("sx-str", m.end()) if (m := STR.match(code, i)) else ("sx-str", n)
        elif c == "'":
            if m := CHAR.match(code, i):
                hit = ("sx-str", m.end())
            elif m := LIFETIME.match(code, i):
                hit = ("sx-ty", m.end())
        elif c in "br" and word_start(i):
            # Prefixed literals: r"..", r#".."#, b"..", br"..", b'x'.
            if m := RAW_STR.match(code, i):
                close = '"' + m.group(1)
                end = code.find(close, m.end())
                hit = ("sx-str", n if end < 0 else end + len(close))
            elif (m := STR.match(code, i)) or (m := CHAR.match(code, i)):
                hit = ("sx-str", m.end())
        if hit is None and "0" <= c <= "9" and word_start(i):
            # x.0 is a tuple field, not a number; 1..5 is a range of numbers.
            if not (code[i - 1:i] == "." and code[i - 2:i - 1] != "."):
                m = NUMBER.match(code, i)
                hit = ("sx-num", m.end())
        if hit is None and (c.isascii() and c.isalpha() or c == "_") and word_start(i):
            m = IDENT.match(code, i)
            word, end = m.group(0), m.end()
            if code[end:end + 1] == "!" and code[end:end + 2] != "!=" and word not in KEYWORDS:
                hit = ("sx-mac", end + 1)
            elif word in KEYWORDS:
                hit = ("sx-kw", end)
            elif word[0].isupper() or word in PRIMITIVES:
                hit = ("sx-ty", end)
            else:
                hit = (None, end)
        if hit is None:
            plain.append(c)
            i += 1
            continue
        cls, end = hit
        if cls is None:
            plain.append(code[i:end])
        else:
            yield from flush()
            yield cls, code[i:end]
        i = end
    yield from flush()


def rust(code):
    out = []
    for cls, text in tokens(code):
        out.append(html.escape(text, quote=False) if cls is None else _span(cls, text))
    return "".join(out)


# Compiler output, line by line, colored the way cargo colors a terminal.
LEVEL = re.compile(r"^(error|warning|help|note)(\[[A-Z]\d+\])?:")
SUB = re.compile(r"^(\s*)(= )(note|help):")
GUTTER = re.compile(r"^(\s*\d*\s*)\|")
ARROW = re.compile(r"^(\s*)(-->|:::)")
LEVEL_CLASS = {"error": "cc-err", "warning": "cc-warn", "help": "cc-help", "note": "cc-note"}


def _compiler_line(line):
    esc = lambda s: html.escape(s, quote=False)
    if m := LEVEL.match(line):
        label, rest = line[:m.end()], line[m.end():]
        return _span(LEVEL_CLASS[m.group(1)], label) + (f'<span class="cc-msg">{esc(rest)}</span>' if rest.strip() else esc(rest))
    if m := SUB.match(line):
        cls = "cc-help" if m.group(3) == "help" else "cc-note"
        lead = esc(m.group(1)) + _span("cc-loc", m.group(2)) + _span(cls, m.group(3) + ":")
        rest = line[m.end():]
        return lead + (_span("cc-note", rest) if cls == "cc-note" else esc(rest))
    if m := ARROW.match(line):
        return esc(m.group(1)) + _span("cc-loc", m.group(2)) + esc(line[m.end():])
    if line.strip() == "...":
        return _span("cc-loc", line)
    if m := GUTTER.match(line):
        head, rest = line[:m.end()], line[m.end():]
        if head.strip() != "|":
            # A source line: number and bar in blue, the code itself plain.
            return _span("cc-loc", head) + esc(rest)
        # A marker line: carets, dashes and connectors, then their label.
        # A marker line: carets, dashes and connectors, then their label,
        # all in blue. A bare label under connectors stays plain.
        mk = re.match(r"[\s|^\-+_~/\\]*", rest).group(0)
        if not mk.strip():
            return _span("cc-loc", head) + esc(rest)
        return _span("cc-loc", line)
    return esc(line)


def compiler(text):
    return "\n".join(_compiler_line(l) for l in text.split("\n"))


def plain_text(markup):
    """The text a reader would copy out of the markup."""
    return html.unescape(re.sub(r"<[^>]+>", "", markup))
