# Compatibility
(authored by agents unless marked 🧑)

- `src/main.rs` reads `KATEX_VERSION`, defaulting to 0.16.4 for released mdbook-katex
  - set `KATEX_VERSION=0.18.5` for the katex-rs migration
  - CSS and every font download use the same version
- 🧑 “... other mdbook_* repos of mine to track the latest mdbook version ...” sets this repository's upgrade context; the downloader itself does not invoke mdBook.
