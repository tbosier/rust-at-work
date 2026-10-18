"""Compile and run every guide snippet; capture outputs for the build.

    python3 tools/verify.py

examples/*.rs must compile and run; their stdout goes to out/<name>.txt.
fail/*.rs must fail to compile; the compiler's message goes to out/fail_<name>.txt.
"""
import fcntl
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SNIP = ROOT / "snippets"
OUT = SNIP / "out"
ENV = dict(os.environ, CARGO_HOME=str(ROOT / ".cargo-home"), CARGO_TERM_COLOR="never")


def run(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, env=ENV, capture_output=True, text=True)


def main():
    OUT.mkdir(exist_ok=True)
    # Several writers may run this at once; the fail crate is shared.
    lock = open(ROOT / ".verify.lock", "w")
    fcntl.flock(lock, fcntl.LOCK_EX)
    bad = 0
    r = run(["cargo", "build", "-q", "--examples"], SNIP)
    if r.returncode != 0:
        print(r.stderr)
        sys.exit("examples failed to build")
    r = run(["cargo", "clippy", "-q", "--examples", "--", "-D", "warnings"], SNIP)
    if r.returncode != 0:
        print(r.stderr)
        sys.exit("examples have clippy warnings")
    for ex in sorted((SNIP / "examples").glob("*.rs")):
        exe = SNIP / "target" / "debug" / "examples" / ex.stem
        r = subprocess.run([str(exe)], cwd=SNIP / "out", capture_output=True, text=True)
        if r.returncode != 0:
            print(f"FAIL run {ex.name}: {r.stderr}")
            bad += 1
        (OUT / f"{ex.stem}.txt").write_text(r.stdout)
    crate = SNIP / "failcrate"
    manifest = (crate / "Cargo.toml").read_text()
    lockfile = (crate / "Cargo.lock").read_text()
    mainfile = (crate / "src" / "main.rs").read_text()
    for f in sorted((SNIP / "fail").glob("*.rs")):
        # Wiki examples are typed in the learner's snippets project. The one
        # error in set up comes from their ticket project.
        name = "kettle_creek" if f.stem == "t1_mut" else "snippets"
        (crate / "Cargo.toml").write_text(re.sub(r'name = "\w+"', f'name = "{name}"', manifest, count=1))
        shutil.copy(f, crate / "src" / "main.rs")
        r = run(["cargo", "build"], crate)
        if r.returncode == 0:
            print(f"FAIL {f.name} compiled, but it should not")
            bad += 1
            continue
        lines = [l for l in r.stderr.splitlines() if not l.strip().startswith("Compiling ")]
        text = "\n".join(lines).strip() + "\n"
        text = re.sub(r"\(/[^)]*\)", "", text)
        text = text.replace(str(Path.home()), "~")
        (OUT / f"fail_{f.stem}.txt").write_text(text)
    (crate / "Cargo.toml").write_text(manifest)
    (crate / "Cargo.lock").write_text(lockfile)
    (crate / "src" / "main.rs").write_text(mainfile)
    print(f"{len(list((SNIP / 'examples').glob('*.rs')))} examples, "
          f"{len(list((SNIP / 'fail').glob('*.rs')))} expected failures, {bad} problems")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
