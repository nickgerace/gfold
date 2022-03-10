#!/usr/bin/env bash
set -e

function log {
    if [ ! "$1" ] || [ "$1" == "" ]; then
        die "internal error: log message empty: please file an issue: https://github.com/nickgerace/gfold/issues/new"
    fi
    echo "[gfold-uninstaller] $1"
}

function die {
    if [ ! "$1" ] || [ "$1" == "" ]; then
        die "internal error: error message empty: please file an issue: https://github.com/nickgerace/gfold/issues/new"
    fi
    log "error: $1"
    exit 1
}

function perform-uninstall {
    for FILE in "/tmp/gfold" "/usr/local/bin/gfold"; do
        if [ -f "$FILE" ]; then
            rm "$FILE"
            log "deleted $FILE"
        fi
    done
    log "uninstallation and cleanup has completed successfully"

    if [ -f $HOME/.config/gfold.toml ]; then
        log "you may want to delete or backup the config file: $HOME/.config/gfold.toml"
    fi
}

perform-uninstall