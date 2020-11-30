# ips
A more optimized rewrite of [ipscoreboard](https://github.com/neelayjunnarkar/ipscoreboard).

## Installation
### Dependencies
- Rust
- Postgres

### Install
```bash
git clone https://github.com/nicolaschan/ips
cd ips
cargo install diesel_cli --no-default-features --features postgres
```

Assuming `$POSTGRES_URL` is your Postgres databese URL, which looks something like:
`postgres://user:password@localhost/postgres_db_name` 

Edit `Rocket.toml` to use this database URL.

```
diesel migration run --database-url "$POSTGRES_URL"
cargo run --release
```
