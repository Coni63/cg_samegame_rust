#!/bin/sh
# Endless improvement loop: every pass restarts the nested search from the best DB solution of each
# grid with a new seed, so the DB can only improve. Stop with Ctrl+C.
# usage: ./improve.sh [nested playout width, default 300] [extra solver options...]
NW=${1:-300}
[ $# -gt 0 ] && shift
pass=0
while true; do
    pass=$((pass + 1))
    ./run_all.sh nw=$NW init=db noise=20 seed=$RANDOM$pass "$@"
    python get_result.py | tail -1
done
