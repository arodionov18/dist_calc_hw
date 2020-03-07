FROM rust:1.41.1-stretch AS build

COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /target/release/hw1 .
COPY --from=build /tools/ ./tools
RUN apt-get update && apt-get -y install libpq5
CMD ["./hw1"]