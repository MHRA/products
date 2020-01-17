FROM rust:1.40 AS build

WORKDIR /usr/src/api

COPY . .
# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time
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
  ;

RUN useradd svc

COPY --from=build /usr/src/api/target/release/api /

RUN chown -R svc /api

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

RUN ls -lah

CMD ["/api"]
