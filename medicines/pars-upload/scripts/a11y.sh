#!/bin/bash
port=3000
cross-env NEXT_PUBLIC_DISABLE_AUTH=true yarn dev -- -p $port &
wait-on http://localhost:$port
server_pid=$!
yarn run pa11y-ci
test_exit_code=$?
kill $server_pid
exit $test_exit_code
