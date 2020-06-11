FROM rust:1.43.1-stretch AS build
ARG CRATE
COPY . .
RUN cargo fetch --manifest-path ${CRATE}/Cargo.toml
RUN rustup component add rustfmt --toolchain 1.43.1-x86_64-unknown-linux-gnu
RUN cargo build --manifest-path ${CRATE}/Cargo.toml --bin ${CRATE} 
RUN cp ${CRATE}/target/debug/${CRATE} /${CRATE}

FROM debian:buster-slim
RUN apt-get update && apt-get -y install libpq5
RUN apt-get install -y ca-certificates

RUN update-ca-certificates
COPY --from=build /${CRATE} .
RUN chmod +x ./tools/wait.sh