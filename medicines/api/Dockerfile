FROM rust:1.40 AS build

WORKDIR /usr/src/api

COPY . .

RUN cargo build --release

# ---------------------------------
FROM debian:buster-slim AS release

RUN apt-get update && apt-get install -y \
  tini \
  ;

COPY --from=build /usr/src/api/target/release/api /

RUN useradd svc

RUN chown -R svc /api

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/api"]
