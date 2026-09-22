The core MVP is complete and clean, but these planned capabilities remain:

  - Detect conflicted bookmarks, not only conflicted stack heads.
  - Report stale or missing workspaces.
  - Use jj’s effective snapshot configuration instead of the fixed 1 MiB threshold.
  - List exact unsnapshotted paths rather than the generic working-copy changes.
  - Add structured JSON output.
  - Add deeper end-to-end tests for remote reachability and shared workspaces.

  What is finished: repository deduplication, filesystem-only detection, local-only stack detection, bookmark-aware actions, root-workspace scanning, failure isolation, conflict
  resolution, divergence cleanup, and explicit detection of divergent change IDs at working copies or local bookmarks.
