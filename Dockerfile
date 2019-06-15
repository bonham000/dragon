FROM rustlang/rust:nightly AS builder

ADD . ./

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cp $(which diesel) target/release/

# RUN mkdir target
# RUN mkdir target/release
# RUN cp diesel target/release/

# RUN ["sh", "-c", "cd target/release && ls -a && cd ../../ && pwd"]

# RUN mkdir target
# RUN mkdir target/release
# RUN cp x.sh target/release

FROM rustlang/rust:nightly

# COPY --from=builder \
#   /target/release/dragon \
#   /usr/local/bin/

# COPY --from=builder /target/release/x.sh /usr/local/bin
COPY --from=builder /src /usr/local/
COPY --from=builder /migrations /usr/local/
COPY --from=builder /Cargo.lock /usr/local/
COPY --from=builder /Cargo.toml /usr/local/
COPY --from=builder /diesel.toml /usr/local/
COPY --from=builder /RustConfig /usr/local/
COPY --from=builder /target/release/dragon /usr/local/bin
COPY --from=builder /target/release/diesel /usr/local/bin

# WORKDIR /root

WORKDIR /usr/local

# CMD ROCKET_PORT=$PORT /usr/local/bin/dragon

# CMD ["sh", "-c", "ROCKET_PORT=$PORT ./bin/x.sh"]
CMD ["sh", "-c", "./bin/diesel && ROCKET_PORT=$PORT ./bin/dragon"]

# CMD ["sh", "-c", "./target/release/diesel setup && ROCKET_PORT=$PORT ./target/release/dragon"]

# web: ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$PORT ./target/release/dragon
# release: ./target/release/diesel setup && ./target/release/diesel migration run