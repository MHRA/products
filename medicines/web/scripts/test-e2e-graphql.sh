#!/bin/bash
port=4000
cross-env USE_GRAPHQL=true yarn dev -p $port &
wait-on http://localhost:$port
server_pid=$!
ENV_VARS=$(cat .env | sed '/^$/d' | tr "\n" "," | sed 's/,$/ /g')
yarn run cypress run --env $ENV_VARS --spec cypress/integration/graphql_tests.js --config baseUrl=http://localhost:$port
test_exit_code=$?
kill $server_pid
exit $test_exit_code
