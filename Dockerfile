# Build the rust project and run the binary
FROM rust:1.36 as be_builder
RUN mkdir -p /usr/src/xmithd_backend
WORKDIR /usr/src/xmithd_backend
COPY . ./
RUN cargo build --release
ENV RUST_LOG="xmithd_backend=debug,actix_web=info"
EXPOSE 3001
CMD ["./target/release/xmithd_backend"]

