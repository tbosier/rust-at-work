#!/bin/sh
# Regenerate every ticket's expected output into out/tN/.
set -e
cd "$(dirname "$0")"
export CARGO_HOME="$(pwd)/../.cargo-home"
cargo build -q --release
rm -rf out
for t in t1 t2 t3 t4 t5; do
  echo "== $t"
  ./target/release/kettle --out "out/$t" "$t"
done
echo "== t6"
./target/release/kettle --out out/t6 t6 --month 2026-09
