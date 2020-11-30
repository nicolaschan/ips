FROM rustlang/rust:nightly-stretch

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/ips
COPY . .

RUN cargo install --path .

CMD ["ips"]
