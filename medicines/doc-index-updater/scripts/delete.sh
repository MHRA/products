#!/bin/sh

file_name=$1

if [ -z "$file_name" ]; then
  echo "run with ./delete.sh <CON_ID>"
  exit 1
fi

metadata_storage_name=$(curl "https://mhraproductsdev.search.windows.net/indexes/products-index/docs\?api-key\=CFBCBE8AA11AA871C14001527533870C\&api-version\=2017-11-11\&highlight\=content\&queryType\=full\&%24count\=true\&%24top\=10\&%24skip\=0\&search\=${file_name}%7E1+${file_name}%5E4\&scoringProfile\=preferKeywords\&searchMode\=all" | jq '.value[].metadata_storage_name' | sed -e 's/\"//' | sed -e 's/\"//')
echo metadata_storage_name: "$metadata_storage_name"

# Run delete job and get the job id
job_id=$(curl --user username:password -X "DELETE" "http://localhost:8000/documents/$file_name" | jq '.id' | sed -e 's/\"//' | sed -e 's/\"//')
echo JobId: "$job_id"

# Check status 5 times with 5 second gap
for i in 1 2 3 4 5; do
  echo
  echo Get status - try $i
  result=$(curl --user username:password -X "GET" "http://localhost:8000/jobs/${job_id}")
  echo "$result"
  status=$(echo "$result" | jq '.status' | sed -e 's/\"//' | sed -e 's/\"//')
  if [ "$status" = "Done" ]; then
    break # Success
  fi
  sleep 5
done
if [ "$status" != "Done" ]; then
  echo "FAIL: Didn't report as deleted from index"
  exit 1
fi

# Check blob is deleted
blob=$(curl "https://mhraproductsdev.blob.core.windows.net/prod-docs/$metadata_storage_name")

case "$blob" in
*"<Code>ResourceNotFound</Code>"*)
  echo "SUCCESS: File deleted from blob storage."
  ;;
*)
  echo "FAIL: blob not deleted."
  exit 1
  ;;
esac
