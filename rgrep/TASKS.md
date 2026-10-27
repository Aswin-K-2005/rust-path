# rgrep — weeks 1–2

`rgrep [flags] <pattern> <path>`

## Tasks
- [x] Parse args by hand from `std::env::args()` into a `Config` struct
- [x] Search one file, print matching lines
- [ ] `-i` case-insensitive
- [ ] `-n` line numbers
- [ ] `-v` invert match
- [ ] `-r` recurse directories (`std::fs::read_dir`, no walkdir crate)
- [ ] Highlight matches in color (ANSI codes by hand)
- [ ] Custom `enum RgrepError` + `impl From<io::Error>` + `impl Display`; no `unwrap()` outside tests
- [ ] Unit tests for the search fn; one integration test in `tests/`
- [ ] Replace hand parsing with `clap` (derive) — note what got simpler

## Done when
- [ ] Works on a real tree (try `~/coding`)
- [ ] clippy + tests green
- [ ] You can explain every `&`, `&mut` and lifetime in the code
- [ ] Post: `posts/01-rgrep.md`
