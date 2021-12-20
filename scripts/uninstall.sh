#!/usr/bin/env bash
set -e

function delete-binary {
    if [ -f "$1" ]; then
        rm "$1"
    fi
}

delete-binary /tmp/gfold
delete-binary /usr/local/bin/gfold
echo "gfold has been removed from your system"