FROM rust:1.80 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/rundaemon /usr/local/bin/rundaemon
COPY examples/sample.yml /app/sample.yml
CMD ["rundaemon", "/app/sample.yml"]
