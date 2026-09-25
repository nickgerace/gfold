The core MVP is complete and clean, but these planned capabilities remain:

  - Use jj’s effective snapshot configuration instead of the fixed 1 MiB threshold.
  - List exact unsnapshotted paths rather than the generic working-copy changes.
  - Add structured JSON output.
  - Add deeper end-to-end tests for remote reachability and shared workspaces.

What is finished:

  - Stale or missing workspace reporting.
  - Repository deduplication.
  - Filesystem-only detection.
  - Local-only stack detection.
  - Bookmark-aware actions.
  - Root-workspace scanning.
  - Failure isolation.
  - Conflict resolution.
  - Divergence cleanup.
  - Explicit detection of divergent change IDs at working copies or local bookmarks.
  - Detection of conflicted local and remote bookmarks.
