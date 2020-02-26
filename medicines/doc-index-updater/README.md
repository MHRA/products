### doc-index-updater

## To run locally (by tunneling redis connection to Azure over TLS):

- install `stunnel` with homebrew:

```
brew install stunnel
```

- start stunnel in a new terminal window:

```bash
cat <<EOF > stunnel.conf
debug = 7
foreground = yes

[doc_index_updater]
client = yes
accept = 127.0.0.1:6379
connect = doc-index-updater-dev.redis.cache.windows.net:6380
EOF
stunnel stunnel.conf
```

- run the service:

```bash
REDIS_ADDR=redis://:<password>@127.0.0.1:6379/ ./target/debug/doc_index_updater # replace <password> with real password
```
