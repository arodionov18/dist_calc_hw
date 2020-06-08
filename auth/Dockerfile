FROM rust:1.41.1-stretch AS build

COPY auth/Cargo.lock auth/Cargo.lock
COPY auth/Cargo.toml auth/Cargo.toml
COPY auth/diesel.toml auth/diesel.toml
COPY auth/.env auth/.env
COPY auth/src auth/src
COPY auth/migrations auth/migrations
RUN cd auth && cargo fetch && cargo build

COPY auth/tools/ auth/tools

FROM debian:buster-slim
COPY --from=build auth/target/debug/auth .
COPY --from=build auth/tools/ ./tools
RUN apt-get update && apt-get -y install libpq5
RUN chmod +x ./tools/wait.sh
CMD ["./auth"]