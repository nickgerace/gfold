#!/usr/bin/env bash
if [ "$REPOPATH" = "" ]; then
    echo "must execute script via make target from repository root"
    exit 1
fi

( cd $REPOPATH; cargo build --release )

OLD=$(which gfold)
NEW=$REPOPATH/target/release/gfold

function run {
	echo "============================================================="
	time $OLD $1
	echo "- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
	time $NEW -i $1
	echo "============================================================="
}

run "/"
run "$HOME/"
run "$HOME/src"
