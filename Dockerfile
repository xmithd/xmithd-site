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
ENV RUST_LOG="xmithd_backend=info"
EXPOSE 3001
CMD ["./target/release/xmithd_backend"]

# Cannot run from alpine so easily because rust compiles agains glibc and alpine doesn't have it.
# FROM alpine:latest
# # If you need certificates, uncomment the following line
# # RUN apk --no-cache add ca-certificates
# RUN mkdir -p /usr/local/xmithd_app/server/static
# RUN mkdir -p /usr/local/xmithd_app/ui/build
# COPY --from=be_builder /usr/src/xmithd_backend/target/release/xmithd_backend /usr/local/xmithd_app/server/xmithd_backend
# COPY --from=be_builder /usr/src/xmithd_backend/static /usr/local/xmithd_app/server/static
# COPY --from=ui_builder /usr/src/xmithd_ui/build /usr/local/xmithd_app/ui/build/
# ENV RUST_LOG="xmithd_backend=info"
# WORKDIR /usr/local/xmithd_app/server
# EXPOSE 3001
# CMD ["./xmithd_backend"]
