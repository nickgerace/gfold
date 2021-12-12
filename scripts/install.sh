#!/usr/bin/env bash
set -e

for BINARY in "jq" "wget" "curl"; do
    if ! [ "$(command -v ${BINARY})" ]; then
        echo "error: \"$BINARY\" must be installed and in PATH"
        exit 1
    fi
done

INSTALL_OS="unknown"
if [ "$(uname -s)" = "Linux" ] && [ "$(uname -m)" = "x86_64" ]; then
    echo "assuming glibc (GNU) and not another libc (e.g. musl-libc)"
    INSTALL_OS="linux-gnu"
elif [ "$(uname -s)" = "Darwin" ] && [ "$(uname -m)" = "x86_64" ]; then
    INSTALL_OS="darwin"
else
    echo "error: must execute on Linux or Darwin x86_64 host"
    exit 1
fi

LATEST=$(curl -s https://api.github.com/repos/nickgerace/gfold/releases/latest | jq -r ".tag_name")
if [ -f /tmp/gfold ]; then rm /tmp/gfold; fi
wget -O /tmp/gfold https://github.com/nickgerace/gfold/releases/download/$LATEST/gfold-$INSTALL_OS-amd64
chmod +x /tmp/gfold
sudo mv /tmp/gfold /usr/local/bin/gfold
echo "gfold has been installed to /usr/local/bin/gfold"
