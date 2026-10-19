"""python3 tools/contrast.py: WCAG contrast for every color pair that meets on
the page, in the light and the dark theme, read from src/base.css.

Exits 1 if a pair misses its target, or if the two themes do not set the
same color tokens.
"""
import re
import sys
from pathlib import Path

CSS = (Path(__file__).resolve().parent.parent / "src" / "base.css").read_text()


def tokens(block):
    return dict(re.findall(r"--([\w-]+):\s*(#[0-9a-fA-F]{6})\s*;", block))


light = tokens(re.search(r":root\s*\{(.*?)\}", CSS, re.S).group(1))
dark = tokens(re.search(r':root\[data-theme="dark"\]\s*\{(.*?)\}', CSS, re.S).group(1))
system_dark = tokens(re.search(r':root:not\(\[data-theme="light"\]\)\s*\{(.*?)\}', CSS, re.S).group(1))


def lum(hexc):
    def ch(v):
        v /= 255
        return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4
    r, g, b = (int(hexc[i:i + 2], 16) for i in (1, 3, 5))
    return 0.2126 * ch(r) + 0.7152 * ch(g) + 0.0722 * ch(b)


def ratio(a, b):
    la, lb = sorted((lum(a), lum(b)), reverse=True)
    return (la + 0.05) / (lb + 0.05)


def lab(hexc):
    def f(t):
        return t ** (1 / 3) if t > 0.008856 else 7.787 * t + 16 / 116
    def ch(v):
        v /= 255
        return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4
    r, g, b = (ch(int(hexc[i:i + 2], 16)) for i in (1, 3, 5))
    x = (0.4124 * r + 0.3576 * g + 0.1805 * b) / 0.95047
    y = 0.2126 * r + 0.7152 * g + 0.0722 * b
    z = (0.0193 * r + 0.1192 * g + 0.9505 * b) / 1.08883
    return 116 * f(y) - 16, 500 * (f(x) - f(y)), 200 * (f(y) - f(z))


def delta_e(a, b):
    return sum((p - q) ** 2 for p, q in zip(lab(a), lab(b))) ** 0.5


# (what, foreground token, background token, target)
TEXT, UI = 4.5, 3.0
PAIRS = [("body text", "ink", bg, TEXT) for bg in ("paper", "sheet", "sea", "quiz-bg", "magenta-wash", "ok-wash", "bad-wash")]
PAIRS += [("secondary text", "ink-2", bg, TEXT) for bg in ("paper", "sheet", "sea", "quiz-bg")]
PAIRS += [("links, headings", "depth", bg, TEXT) for bg in ("paper", "sheet", "sea", "quiz-bg")]
PAIRS += [("flag heading", "magenta", "magenta-wash", TEXT)]
PAIRS += [("quiz verdict", "ok", "ok-wash", TEXT), ("quiz verdict", "bad", "bad-wash", TEXT)]
PAIRS += [("button text", "on-depth", "depth", TEXT), ("button hover", "on-depth", "depth-dark", TEXT)]
PAIRS += [(f"syntax {t}", t, bg, TEXT) for t in ("ink", "sx-kw", "sx-ty", "sx-str", "sx-com") for bg in ("sea", "sheet")]
PAIRS += [(f"compiler {t}", t, bg, TEXT) for t in ("ink", "cc-err", "cc-warn", "cc-loc", "cc-help", "cc-note") for bg in ("sea", "paper")]
PAIRS += [("focus ring", "depth", bg, UI) for bg in ("paper", "sheet", "sea", "quiz-bg")]
PAIRS += [("answer border", "ok", "quiz-bg", UI), ("answer border", "bad", "quiz-bg", UI),
          ("feedback border", "ok", "ok-wash", UI), ("feedback border", "bad", "bad-wash", UI),
          ("flag rule", "magenta", "magenta-wash", UI),
          ("control border", "sea-edge", "paper", UI), ("control border", "depth", "paper", UI)]


def main():
    bad = 0
    if set(light) != set(dark) or dark != system_dark:
        print("themes differ in the tokens they set:", sorted(set(light) ^ set(dark)) or "system and picked dark differ")
        bad += 1
    print(f"{'pair':44} {'light':>6} {'dark':>6}  target")
    for what, fg, bg, target in PAIRS:
        rl, rd = ratio(light[fg], light[bg]), ratio(dark[fg], dark[bg])
        miss = rl < target or rd < target
        bad += miss
        print(f"{what + ': ' + fg + ' on ' + bg:44} {rl:6.2f} {rd:6.2f}  {target}{'  MISS' if miss else ''}")
    # Syntax hues and compiler hues must not read as the same color.
    print("\nclosest syntax/compiler pair (CIE76 delta E; under 20 reads as close):")
    for name, theme in (("light", light), ("dark", dark)):
        pairs = [(delta_e(theme[a], theme[b]), a, b) for a in ("sx-kw", "sx-ty", "sx-str")
                 for b in ("cc-err", "cc-warn", "cc-loc", "cc-help", "ok", "bad")]
        inner = [(delta_e(theme[a], theme[b]), a, b) for a, b in (("sx-kw", "sx-ty"), ("sx-kw", "sx-str"), ("sx-ty", "sx-str"))]
        d, a, b = min(pairs)
        d2, a2, b2 = min(inner)
        print(f"  {name}: {a} vs {b} {d:.1f}; closest two syntax hues {a2} vs {b2} {d2:.1f}")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
