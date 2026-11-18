# llm-rs: run an LLM with Rust code I wrote myself

Learn Rust and how LLMs work at the same time, by building an inference engine from scratch.
Base: Karpathy's llama2.c (github.com/karpathy/llama2.c), ported to Rust by hand.
Hardware: RTX 4070 laptop (8GB VRAM) + 32GB RAM. Everything here runs on CPU first.

## Tasks
- [ ] Math kernels on `&[f32]` slices: `matmul`, `softmax`, `rmsnorm`, each with a unit test (Rust: slices, iterators, `assert!`)
- [ ] Read `stories15M.bin`: config header + weights into a `struct Weights` (Rust: `fs::read`, `f32::from_le_bytes`, `chunks_exact`)
- [ ] Forward pass for one token: embedding → attention → FFN → logits (LLM: what a transformer layer actually does)
- [ ] Tokenizer: load `tokenizer.bin`, encode/decode (reuse ideas from `~/coding/rust/tokenize`)
- [ ] Generate text greedily: the model writes a story 🎉 (milestone: "my Rust code makes a model talk")
- [ ] KV cache: measure tokens/sec before vs after (LLM: why long context costs memory)
- [ ] Sampling: temperature + top-p (LLM: why outputs vary)
- [ ] Threads: parallel matmul with `std::thread::scope`, then compare with `rayon` (Rust: Send/Sync, borrowing across threads)
- [ ] int8 quantization: memory, speed and output quality vs f32 (LLM: how big models fit on small machines)
- [ ] Bigger model: `stories110M`, then read a GGUF file header (LLM: the format llama.cpp/Ollama use)

## Concept track (no code, ~1 per week, notes in `posts/`)
- [ ] Memory math: params × bytes per weight (fp16 = 2, q8 = 1, q4 ≈ 0.55) + KV cache size. Work out what fits on this laptop.
- [ ] Mixture of Experts: why a "30B-A3B" model runs as fast as a 3B one but needs RAM for all 30B
- [ ] Offloading: GPU layers vs CPU layers, and why memory bandwidth sets tokens/sec
- [ ] Small vs big models: what a narrow task + verifier loop recovers (links to the local coding agent plan)

## Done when
- [ ] `cargo run --release` generates a coherent story from stories15M with my own code
- [ ] clippy + tests green
- [ ] Post: `posts/llm-rs.md` with a table: f32 vs int8 vs threads → tokens/sec and memory
