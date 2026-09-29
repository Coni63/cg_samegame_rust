#!/bin/sh
# Solve every validation grid (one per hash) with the given solver options, saving results to the DB.
# usage: ./run_all.sh width=1000 nw=100 ...
export PATH="/c/Program Files/DB Browser for SQLite:$PATH"
for i in 6 12 13 14 7 16 17 18 19 8 21 22 23 24 9 26 27 28 29 10; do
    ./target/release/cg_samegame_rust.exe testcases/test$i.json "$@" 2>&1 >/dev/null | grep -E "^(target .* ->|score:)" | tr '\n' ' '
    echo " test$i"
done
