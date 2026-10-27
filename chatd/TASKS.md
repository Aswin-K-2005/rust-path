# chatd — weeks 9–11

Multi-client TCP chat server. Clients: `nc localhost 7878`.

## Tasks
### v1: threads
- [ ] `TcpListener`, one thread per client
- [ ] Shared client list `Arc<Mutex<Vec<...>>>`, broadcast messages
- [ ] Usernames (`/nick`), join/leave messages
- [ ] Try `Rc` instead of `Arc` — read the compile error, write down why (Send)

### v2: tokio
- [ ] Rewrite with `tokio::net`, `tokio::sync::broadcast`
- [ ] Rooms: `/join <room>`
- [ ] Graceful shutdown on Ctrl-C (`tokio::signal`)
- [ ] Handle client disconnect without crashing the server

## Done when
- [ ] 3 terminals chatting in 2 rooms
- [ ] clippy + tests green
- [ ] You can explain Send vs Sync and why a `MutexGuard` can't cross `.await`
- [ ] Post: `posts/04-chatd.md`
