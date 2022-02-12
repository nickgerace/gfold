#!/usr/bin/env bash
set -e

function log {
    if [ ! "$1" ] || [ "$1" == "" ]; then
        die "internal error: log message empty: please file an issue: https://github.com/nickgerace/gfold/issues/new"
    fi
    echo "[gfold-installer] $1"
}

function die {
    if [ ! "$1" ] || [ "$1" == "" ]; then
        die "internal error: error message empty: please file an issue: https://github.com/nickgerace/gfold/issues/new"
    fi
    log "error: $1"
    exit 1
}

function check-dependencies {
    for BINARY in "jq" "wget" "curl"; do
        if ! [ "$(command -v ${BINARY})" ]; then
            die "\"$BINARY\" must be installed and in PATH"
        fi
    done
}

function perform-install {
    local INSTALL_OS
    if [ "$(uname -s)" = "Linux" ] && [ "$(uname -m)" = "x86_64" ]; then
        INSTALL_OS="linux-gnu"
    elif [ "$(uname -s)" = "Darwin" ] && [ "$(uname -m)" = "x86_64" ]; then
        INSTALL_OS="darwin"
    else
        die "must execute on Linux or Darwin x86_64 host (for more installation methods, refer to the docs: https://github.com/nickgerace/gfold)"
    fi

    LATEST=$(curl -s https://api.github.com/repos/nickgerace/gfold/releases/latest | jq -r ".tag_name")
    if [ -f /tmp/gfold ]; then
        rm /tmp/gfold
    fi
    wget -O /tmp/gfold https://github.com/nickgerace/gfold/releases/download/$LATEST/gfold-$INSTALL_OS-amd64
    chmod +x /tmp/gfold

    if [ -f /usr/local/bin/gfold ]; then
        rm /usr/local/bin/gfold
    fi
    mv /tmp/gfold /usr/local/bin/gfold

    if [ $INSTALL_OS = "linux-gnu" ]; then
        log "assuming glibc (GNU) and not another libc (e.g. musl-libc)"
        log "if using another libc, you may need to choose another installation method"
        log "for more information, refer to the docs: https://github.com/nickgerace/gfold"
    fi
    log "gfold has been installed to /usr/local/bin/gfold"
}

check-dependencies
perform-install