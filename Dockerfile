FROM rust:latest AS BUILDER
WORKDIR /app/
COPY ./ ./
RUN cargo build -r -p server

FROM debian:stable-slim
WORKDIR /opt/stream-url
COPY --from=BUILDER /app/target/release/server .
EXPOSE 80
ENTRYPOINT [ "./server" ]
