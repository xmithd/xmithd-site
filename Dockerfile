FROM node:10.16 as ui_builder
RUN mkdir -p /usr/src/xmithd_ui
WORKDIR /usr/src/xmithd_ui
COPY ./ui/ ./
RUN npm install && npm run build

# Build the rust project and run the binary
FROM rust:1.36 as be_builder
RUN mkdir -p /usr/src/xmithd_backend
WORKDIR /usr/src/xmithd_backend
COPY ./server/ ./
RUN cargo build --release
RUN mkdir -p /usr/src/ui/build
COPY --from=ui_builder /usr/src/xmithd_ui/build/ /usr/src/ui/build
ENV RUST_LOG="xmithd_backend=debug,actix_web=info"
EXPOSE 3001
CMD ["./target/release/xmithd_backend"]

