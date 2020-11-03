FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
  tini \
  ca-certificates \
  libssl-dev \
  openssl \
  ;

RUN useradd svc

ENV PORT=8000

COPY --chown=svc ./target/release/api /

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/api"]
