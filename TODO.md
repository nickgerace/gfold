# TODO

- Look up libgfold usages, if low, consider consolidating gfold back into one crate (also consider removing bin and lib differences and flatten crate structure or add everything to a "crates" directory)
- Add benchmark that grabs binary from github or uses a local instance (or find another way to test the old release version... maybe latest tag on github or a cargo test-based bench rather than an xtask that calls subcommands?)
- Do not have display modes and add tunable display
- Look up toml maintainership and consider own config structure
- Allow multiple config paths (maybe the same as alacritty or kitty)
- Add ability to have "git diff" type of a view mode (read: I am saying to add a pager, holy shit Nick)
- Tree implementation? (like the tree command)
- Add built with nix badge and nix development instructions
- Use libgfold-v5 for development mainly
- Consolidate Cli, EntryConfig and Config into one struct
- Try burntsushi walkdir as an option
- Research if libgfold is actually used (architectural benefit: I can log everywhere and use anyhow)
- Research how upstream libraries handle errors from other libraries
- Research justfile as a make replacement (including windows support)
