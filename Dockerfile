# Cargo build stage to build dependencies separately

FROM rust:latest AS builder

# RUN apt-get update
# RUN apt-get install musl-tools -y
# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"Hello, world!\")}" > src/main.rs

# Add compiler (if not included in the image) - uncomment if build machine is not x86_64
# RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target=x86_64-unknown-linux-gnu

# Do some cleanup
RUN rm -f target/x86_64-unknown-linux-gnu/release/deps/xmithd_backend*

# Add our code and build it
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-gnu

## Run the binary from debian:stable-slim as non-root

FROM debian:stable-slim

# Note: currently addgroup is not included in the slim image
# solution: use groupadd and useradd
# RUN apt-get update && apt-get install -y adduser
# RUN addgroup --gid 1000 xmithd
# RUN adduser --gecos "" --disabled-password --shell /bin/bash --uid 1000 --ingroup xmithd xmithd
RUN groupadd --gid 1000 xmithd
RUN useradd --create-home --shell /bin/bash --uid 1000 --gid xmithd xmithd

WORKDIR /home/xmithd/bin/

RUN mkdir static && mkdir database

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/xmithd_backend .
COPY ./static/ ./static/
COPY config.json .

RUN chown xmithd:xmithd config.json
RUN chown xmithd:xmithd xmithd_backend
RUN chown -R xmithd:xmithd static
ENV RUST_LOG="xmithd_backend=debug,tower-http=info"
EXPOSE 3001
VOLUME /home/xmithd/bin/database

CMD ["./xmithd_backend"]
