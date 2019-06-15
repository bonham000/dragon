FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cp $(which diesel) target/release/

FROM rustlang/rust:nightly

# COPY --from=builder \
#   /target/release/dragon \
#   /usr/local/bin/

COPY --from=builder . /usr/local/

WORKDIR /root

# CMD ROCKET_PORT=$PORT /usr/local/bin/dragon

CMD ["sh", "-c", "cd usr/local && /usr/local/target/release/diesel setup && ROCKET_PORT=$PORT /usr/local/target/release/dragon"]

# web: ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$PORT ./target/release/dragon
# release: ./target/release/diesel setup && ./target/release/diesel migration run