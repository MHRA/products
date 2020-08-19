#!/bin/bash
env USE_GRAPHQL=false yarn dev &
wait-on http://localhost:3000
server_pid=$!
ENV_VARS=$(cat .env | sed '/^$/d' | tr "\n" "," | sed 's/,$/ /g')
yarn run cypress run --env $ENV_VARS --spec cypress/integration/azure_search_tests.js
test_exit_code=$?
kill $server_pid
exit $test_exit_code
