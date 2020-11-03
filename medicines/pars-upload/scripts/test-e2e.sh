#!/bin/bash
port=4000
cross-env NEXT_PUBLIC_DISABLE_AUTH=true yarn build && yarn start -p $port &
wait-on http://localhost:$port
server_pid=$!
yarn run cypress run --env PARS_UPLOAD_URL=$PARS_UPLOAD_URL --config baseUrl=http://localhost:$port
test_exit_code=$?
kill $server_pid
exit $test_exit_code
