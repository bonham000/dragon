FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

FROM rustlang/rust:nightly

COPY --from=builder \
  /target/release/dragon \
  /usr/local/bin/

WORKDIR /root

CMD ROCKET_PORT=$PORT /usr/local/bin/dragon