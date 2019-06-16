FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

# RUN cargo install diesel_cli --no-default-features --features postgres

# RUN cp $(which diesel) target/release/

FROM rustlang/rust:nightly

# COPY --from=builder /src /usr/local/
# COPY --from=builder /migrations /usr/local/
# COPY --from=builder /Cargo.lock /usr/local/
# COPY --from=builder /Cargo.toml /usr/local/
# COPY --from=builder /diesel.toml /usr/local/
# COPY --from=builder /RustConfig /usr/local/
# COPY --from=builder /target/release/diesel /usr/local/bin
COPY --from=builder /target/release/dragon /usr/local/bin

WORKDIR /usr/local

# CMD ["sh", "-c", "./bin/diesel migration run && ROCKET_PORT=$PORT ./bin/dragon"]
CMD ["sh", "-c", "ROCKET_PORT=$PORT ./bin/dragon"]