# rsh — weeks 6–8

A shell, by hand. You built one in C — this is the Rust rewrite.

## Tasks
- [ ] REPL loop: prompt, read line, handle EOF (Ctrl-D)
- [ ] Tokenize commands (reuse/adapt tinylang lexer: quotes, `|`, `<`, `>`)
- [ ] Builtins: `cd`, `exit`, `export`, `pwd`
- [ ] External commands via `std::process::Command`
- [ ] Pipes `a | b | c` with `Command` + `Stdio::piped()`
- [ ] Redirects `>`, `>>`, `<`
- [ ] Rewrite pipes using `nix` (`fork`, `pipe`, `dup2`, `execvp`, `waitpid`)
- [ ] Ctrl-C kills the child, not the shell (signal handling)
- [ ] `$VAR` expansion
- [ ] Exit status in prompt

## Done when
- [ ] `ls | grep rs | wc -l > out.txt` works
- [ ] clippy + tests green (test the parser hard)
- [ ] Post: `posts/03-c-vs-rust-shell.md` — what Rust forced you to do differently
