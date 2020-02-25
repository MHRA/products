
FROM rust:1.40 AS build

WORKDIR /usr/src/api

COPY Cargo.toml .
COPY Cargo.lock .

# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time

# it was borrowed from here: 
# https://github.com/deislabs/krustlet/blob/master/Dockerfile#L7 
RUN mkdir -p ./src/ && \
  echo 'fn main() {}' > ./src/main.rs && \
  echo '' > ./src/lib.rs

RUN cargo fetch

RUN cargo build --release && \
  rm -rf ./target/release/.fingerprint/api-*

# Build real binaries now
COPY ./src ./src

RUN cargo build --release

# ---------------------------------
FROM debian:buster-slim AS release

RUN apt-get update && apt-get install -y \
  tini \
  libssl-dev \
  openssl \
  curl \
  ;

RUN useradd svc

COPY --from=build /usr/src/api/target/release/api /

RUN chown -R svc /api

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/api"]