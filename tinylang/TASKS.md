# tinylang — weeks 3–5

Interpreter for: `let x = 5 + 3 * 2; print x; if x > 3 { print 1; } while ...`

## Tasks
- [ ] `enum Token<'a>` — lexer borrows `&'a str` slices, zero `String` clones
- [ ] Lexer tests: numbers, idents, operators, keywords, errors with line/col
- [ ] `enum Expr { Num(i64), Var(&str), Binary(Box<Expr>, Op, Box<Expr>), ... }`
- [ ] Recursive-descent parser with correct precedence (`*` before `+`)
- [ ] `enum Stmt` — let, print, if, while, blocks
- [ ] Evaluator with `HashMap` environment
- [ ] Your own `trait Eval` and `impl Display` for the AST (pretty-printer)
- [ ] Parse errors as `Result`, never panic on bad input
- [ ] REPL mode + run-a-file mode

## Done when
- [ ] FizzBuzz written in tinylang runs
- [ ] clippy + tests green
- [ ] Checkpoint: rewrite the lexer's `next_token` from memory
- [ ] Post: `posts/02-tinylang.md`
