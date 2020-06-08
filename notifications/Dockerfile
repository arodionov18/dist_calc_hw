FROM rust:1.41.1-stretch AS build

COPY notifications/Cargo.lock notifications/Cargo.lock
COPY notifications/Cargo.toml notifications/Cargo.toml
COPY notifications/.env notifications/.env
COPY notifications/src notifications/src
RUN cd notifications && cargo fetch && cargo build

COPY notifications/tools/ notifications/tools

FROM debian:buster-slim
COPY --from=build notifications/target/debug/notifications .
COPY --from=build notifications/tools/ ./tools
RUN apt-get update && apt-get -y install libpq5
RUN chmod +x ./tools/wait.sh
RUN apt-get install -y ca-certificates
RUN update-ca-certificates
CMD ["./notifications"]