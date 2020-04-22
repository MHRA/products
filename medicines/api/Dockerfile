
FROM rust:1.40 AS build

WORKDIR /usr/src/api

ENV SCCACHE_URL=https://github.com/mozilla/sccache/releases/download/0.2.13/sccache-0.2.13-x86_64-unknown-linux-musl.tar.gz

RUN mkdir -p /opt/sccache && \
  curl -s -L $SCCACHE_URL | tar -xz --strip 1 -C /opt/sccache && \
  chmod +x /opt/sccache/sccache

ARG SCCACHE_AZURE_CONNECTION_STRING
ARG SCCACHE_AZURE_BLOB_CONTAINER=cicache

ENV RUSTC_WRAPPER=/opt/sccache/sccache \
  SCCACHE_AZURE_CONNECTION_STRING=$SCCACHE_AZURE_CONNECTION_STRING \
  SCCACHE_AZURE_BLOB_CONTAINER=$SCCACHE_AZURE_BLOB_CONTAINER

COPY api/Cargo.toml api/Cargo.lock ./

COPY search-client ../search-client

# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time

# it was borrowed from here:
# https://github.com/deislabs/krustlet/blob/master/Dockerfile#L7
RUN mkdir -p ./src/ && \
  echo 'fn main() {}' > ./src/main.rs && \
  echo '' > ./src/lib.rs

RUN cargo build --release && \
  rm -rf ./target/release/.fingerprint/api-*

# Build real binaries now
COPY ./api/src ./src

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