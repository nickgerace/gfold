#!/usr/bin/env bash
set -e

IS_LINUX="false"
if [ "$(uname -s)" = "Linux" ]; then
    IS_LINUX="true"
    sudo -v
fi

function delete-binary {
    if [ -f "$1" ]; then
        if [ "$IS_LINUX" = "true" ]; then
           sudo rm "$1"
        else
           rm "$1"
        fi
    fi
}

delete-binary /tmp/gfold
delete-binary /usr/local/bin/gfold
echo "gfold has been removed from your system"