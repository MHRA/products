#!/bin/bash
yarn dev &
wait-on http://localhost:3000
server_pid=$!
yarn run pa11y-ci
test_exit_code=$?
kill $server_pid
exit $test_exit_code
