#!/usr/bin/env bash
set -e

function perform-uninstall {
    for FILE in "/tmp/gfold" "/usr/local/bin/gfold"; do
        if [ -f "$FILE" ]; then
            rm "$FILE"
            echo "[uninstall-gfold] ✅  deleted $FILE"
        fi
    done
    echo "[uninstall-gfold] ✅  uninstallation/cleanup has completed successfully"

    if [ -f $HOME/.config/gfold/gfold.json ]; then
        echo "[uninstall-gfold] ⚠️  you may want to delete or backup the config file"
        echo "[uninstall-gfold] ⚠️  config file path: $HOME/.config/gfold/gfold.json"
    fi
}

perform-uninstall