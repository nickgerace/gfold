#!/usr/bin/env bash
set -e

if [ "$REPOPATH" = "" ]; then
    echo "must execute script via make target from repository root"
    exit 1
fi

( cd $REPOPATH; cargo build --release )

OLD=$(which gfold)
NEW=$REPOPATH/target/release/gfold

function run {
  for COUNT in {1..4}; do
    echo "


$1 $COUNT
"
    time $OLD $1
    echo "- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
    time $NEW -i $1
  done
}

run "$HOME/"
run "$HOME/src"
