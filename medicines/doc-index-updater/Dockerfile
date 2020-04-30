# Setup rust build environment
FROM rust:1.43.0 AS build-context

ENV SCCACHE_URL=https://github.com/mozilla/sccache/releases/download/0.2.13/sccache-0.2.13-x86_64-unknown-linux-musl.tar.gz

RUN mkdir -p /opt/sccache && \
  curl -s -L $SCCACHE_URL | tar -xz --strip 1 -C /opt/sccache && \
  chmod +x /opt/sccache/sccache

ARG SCCACHE_AZURE_CONNECTION_STRING
ARG SCCACHE_AZURE_BLOB_CONTAINER=cicache

ENV RUSTC_WRAPPER=/opt/sccache/sccache \
  SCCACHE_AZURE_CONNECTION_STRING=$SCCACHE_AZURE_CONNECTION_STRING \
  SCCACHE_AZURE_BLOB_CONTAINER=$SCCACHE_AZURE_BLOB_CONTAINER

RUN apt-get update && apt-get install -y \
  redis \
  ;

RUN rustup component add rustfmt clippy

WORKDIR /usr/src/doc-index-updater

COPY doc-index-updater/Cargo.toml doc-index-updater/Cargo.lock ./

COPY search-client ../search-client

# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time

# it was borrowed from here:
# https://github.com/deislabs/krustlet/blob/master/Dockerfile#L7
RUN mkdir -p ./src/ && \
  echo 'fn main() {}' >./src/main.rs && \
  echo '' >./src/lib.rs

RUN cargo build --release && \
  rm -rf ./target/release/.fingerprint/doc_index_updater-*

# Setup debian release environment
FROM debian:buster-slim AS release-context

RUN apt-get update && apt-get install -y \
  openssl \
  ca-certificates \
  tini \
  ;

RUN useradd svc

ENV PORT=8000

# Build real binaries now, as late as possible
FROM build-context AS build

COPY ./doc-index-updater/src ./src

RUN cargo clippy --release -- -D warnings && \
  cargo test --release && \
  cargo build --release

# Create the release
FROM release-context AS release

COPY --from=build /usr/src/doc-index-updater/target/release/doc_index_updater /

RUN chown -R svc /doc_index_updater

RUN (cd $HOME && \
  mkdir -p .ssh && \
  chown svc .ssh && \
  cd .ssh && \
  echo "GSSAPIAuthentication no" >>config && \
  echo "GSSAPIDelegateCredentials yes" >>config && \
  chown svc config)

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/doc_index_updater"]
