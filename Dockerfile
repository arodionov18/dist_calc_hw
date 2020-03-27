FROM rust:1.41.1-stretch AS build

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY diesel.toml diesel.toml
COPY .env .env
COPY src src
COPY migrations migrations
COPY tools tools
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /target/release/hw1 .
COPY --from=build /tools/ ./tools
RUN apt-get update && apt-get -y install libpq5
RUN chmod +x ./tools/wait.sh
CMD ["./hw1"]