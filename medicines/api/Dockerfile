# Setup rust build environment
FROM rust:1.41.0 AS build-context

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

# Setup debian release environment
FROM debian:buster-slim AS release-context

RUN apt-get update && apt-get install -y \
  tini \
  libssl-dev \
  ;

RUN useradd svc

ARG AZURE_SEARCH_INDEX
ARG AZURE_SEARCH_KEY
ARG AZURE_SEARCH_SERVICE

ENV AZURE_SEARCH_INDEX=${AZURE_SEARCH_INDEX}
ENV AZURE_SEARCH_KEY=${AZURE_SEARCH_KEY}
ENV AZURE_SEARCH_SERVICE=${AZURE_SEARCH_SERVICE}
ENV PORT=8000

# Build real binaries now, as late as possible
FROM build-context AS build

COPY ./src ./src

RUN cargo build --release

# Create the release
FROM release-context AS release

COPY --from=build2 /usr/src/api/target/release/api /

RUN chown -R svc /api

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/api"]
