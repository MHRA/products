FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
  openssl \
  ca-certificates \
  tini \
  ;

RUN useradd svc

ENV PORT=8000

COPY --chown=svc ./target/release/doc_index_updater /

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
