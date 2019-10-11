# Cargo build stage to build dependencies separately

FROM rust:latest as builder

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"Hello, world!\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Do some cleanup
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/xmithd_backend*

# Add our code and build it
COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl 

## Run the binary from alpine as non-root

FROM alpine:latest

RUN addgroup -g 1000 xmithd

RUN adduser -D -s /bin/sh -u 1000 -G xmithd xmithd

WORKDIR /home/xmithd/bin/

RUN mkdir static

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/xmithd_backend .
COPY ./static/ ./static/
COPY config.json .

RUN chown xmithd:xmithd config.json
RUN chown xmithd:xmithd xmithd_backend
RUN chown -R xmithd:xmithd static
ENV RUST_LOG="xmithd_backend=debug,actix_web=info"
EXPOSE 3001

CMD ["./xmithd_backend"]
