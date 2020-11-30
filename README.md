# ips
A more optimized rewrite of [ipscoreboard](https://github.com/neelayjunnarkar/ipscoreboard).

## Installation
### Dependencies
- Rust

### Install
```bash
git clone https://github.com/nicolaschan/ips
cd ips
cargo install diesel_cli
diesel migration run --database-url hits.sqlite
cargo run --release
```
