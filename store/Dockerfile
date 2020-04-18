FROM rust:1.41.1-stretch AS build

COPY store/Cargo.lock store/Cargo.lock
COPY store/Cargo.toml store/Cargo.toml
COPY store/diesel.toml store/diesel.toml
COPY store/.env store/.env
COPY store/src store/src
COPY store/migrations store/migrations
COPY store/tools store/tools
RUN cd store && cargo build --release

FROM debian:buster-slim
COPY --from=build store/target/release/hw1 .
COPY --from=build store/tools/ ./tools
RUN apt-get update && apt-get -y install libpq5
RUN chmod +x ./tools/wait.sh
CMD ["./hw1"]