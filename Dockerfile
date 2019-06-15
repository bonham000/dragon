FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

FROM rustlang/rust:nightly

COPY --from=builder \
  /target/release/dragon \
  /usr/local/bin/

WORKDIR /root

CMD ROCKET_PORT=$PORT /usr/local/bin/dragon

# web: ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$PORT ./target/release/dragon
# release: ./target/release/diesel setup && ./target/release/diesel migration run