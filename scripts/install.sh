#!/usr/bin/env bash
set -e

function check-dependencies {
    for BINARY in "jq" "wget" "curl"; do
        if ! [ "$(command -v ${BINARY})" ]; then
            echo "[install-gfold] 🚫  \"$BINARY\" must be installed and in PATH"
            exit 1
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
        echo "[install-gfold] 🚫  must execute on Linux or Darwin x86_64 host"
        echo "[install-gfold] 🚫  for more installation methods: https://github.com/nickgerace/gfold"
        exit 1
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

    echo "[install-gfold] ✅  gfold has been installed to /usr/local/bin/gfold"
    if [ $INSTALL_OS = "linux-gnu" ]; then
        echo "[install-gfold] ⚠️  assuming glibc (GNU) and not another libc (e.g. musl-libc)"
        echo "[install-gfold] ⚠️  if using another libc, you may need to choose another installation method"
        echo "[install-gfold] ⚠️  for more information: https://github.com/nickgerace/gfold"
    fi
}

check-dependencies
perform-install