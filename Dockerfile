FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

CMD ["sh", "-c", "diesel migration run && ROCKET_PORT=$PORT ./target/release/dragon"]