#!/usr/bin/env bash
set -e

REALPATH=realpath
if [ "$(uname -s)" = "Darwin" ]; then
    REALPATH=grealpath
fi
REPOPATH=$(dirname $(dirname $($REALPATH -s $0)))

( cd $REPOPATH; cargo build --release )

OLD=$HOME/.cargo/bin/gfold
NEW=$REPOPATH/target/release/gfold

function run {
    local BENCH_FILE
    for COUNT in {1..4}; do
        echo "- - - - - - - - - - - - -"
        echo "[OLD]"
        time $OLD -i $1 > /dev/null
        echo "[NEW]"
        time $NEW -i $1 > /dev/null
    done
}

run "$HOME/" "home"
run "$HOME/src" "src"
