# Manual Install

This document contains methods on how to install `gfold` "manually" (i.e. without a package manager or registry).

## Download and Install a Binary on macOS and Linux

Executing the commands in this section requires the following:

- macOS or Linux (GNU, not MUSL) system
- `x86_64 / amd64` architecture
- `bash` shell (or compatible)
- `jq`, `wget` and `curl` installed and in `PATH`

First, let's ensure we have our prerequisite binaries installed.

```bash
for BINARY in "jq" "wget" "curl"; do
    if ! [ "$(command -v ${BINARY})" ]; then
        echo "\"$BINARY\" must be installed and in PATH"
        return
    fi
done
```

Now, let's determine which binary we need to choose based on our platform.

```bash
INSTALL_OS=""
if [ "$(uname -s)" = "Linux" ] && [ "$(uname -m)" = "x86_64" ]; then
    INSTALL_OS="linux-gnu"
elif [ "$(uname -s)" = "Darwin" ] && [ "$(uname -m)" = "x86_64" ]; then
    INSTALL_OS="darwin"
else
    echo "must execute on Linux or Darwin x86_64 (x86_64 / amd64) host"
    return
fi
```

We need to determine to latest tag to build our release URL.

```bash
LATEST=$(curl -s https://api.github.com/repos/nickgerace/gfold/releases/latest | jq -r ".tag_name")
```

With the latest tag and platform determined, we can finally download and install `gfold` to `/usr/local/bin/`.

```bash
# Remove gfold if it is already in /tmp.
if [ -f /tmp/gfold ]; then
    rm /tmp/gfold
fi

# Perform the download.
wget -O /tmp/gfold https://github.com/nickgerace/gfold/releases/download/$LATEST/gfold-$INSTALL_OS-amd64

# Set executable permissions.
chmod +x /tmp/gfold

# Remove gfold if it is already in /usr/local/bin/.
if [ -f /usr/local/bin/gfold ]; then
    rm /usr/local/bin/gfold
fi

# Move gfold into /usr/local/bin/.
mv /tmp/gfold /usr/local/bin/gfold
```

### Uninstalling and Cleaning Up

If you would like to uninstall `gfold` and remove potential artifacts from the method above, execute the following:

```bash
# Remove potential installed and/or downloaded artifacts.
rm /tmp/gfold
rm /usr/local/bin/gfold

# (Optional) remove the configuration file.
rm $HOME/.config/gfold.toml
 ```

## Build From Source Locally On All Platforms

If you want to install from source locally, and not from [crates.io](https://crates.io/crates/gfold), you can clone the repository and build `gfold`.
This should work on all major platforms.

```bash
git clone https://github.com/nickgerace/gfold.git
cd gfold; cargo install --path crates/gfold
```

The commands above were tested on macOS.
Slight modification may be required for your platform, but the flow should be the same: clone, change directory and run
`cargo install`.