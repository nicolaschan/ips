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

Assuming `$POSTGRES_URL` is your Postgres database URL, which looks something like:
`postgres://user:password@localhost/postgres_db_name` 

Edit `Rocket.toml` to use this database URL. 
Then run the migration to create the hits table:

```
diesel migration run --database-url "$POSTGRES_URL"
cargo run --release
```

### Development
For testing and sketchy prod setups, run Postgres in Docker:
```
docker run -p 5432:5432 -d -e POSTGRES_PASSWORD=adgnOl72VEspIwcZh postgres
```
As is, this won't save any state once the container stops.
To save the state when the container stops, use volumes (see [docker/postgres](https://hub.docker.com/_/postgres/)).

Then we have
```
POSTGRES_URL=postgres://postgres:adgnOl72VEspIwcZh@localhost/postgres
```
