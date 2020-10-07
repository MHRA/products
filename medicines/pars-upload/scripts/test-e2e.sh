#!/bin/bash
port=3000
cross-env NEXT_PUBLIC_DISABLE_AUTH=true yarn dev -- -p $port & 
wait-on http://localhost:$port
server_pid=$!
yarn cypress run --env PARS_UPLOAD_URL=$PARS_UPLOAD_URL --config baseUrl=http://localhost:$port
test_exit_code=$?
kill $server_pid
exit $test_exit_code
