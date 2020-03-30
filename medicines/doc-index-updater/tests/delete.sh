file_name=$1

if [ -z "$file_name" ]
then
  echo "run with ./delete.sh <CON_ID>"
  exit 1
fi

job_id=`curl --user username:password -X "DELETE" "http://localhost:8000/documents/$file_name" | jq '.id' | sed -e 's/\"//' | sed -e 's/\"//'`


echo $job_id

for i in 1 2 3 4 5
do
   echo Get status - try $i
   curl --user username:password -X "GET" "http://localhost:8000/jobs/${job_id}"
   echo 
   echo 
   sleep 7
done
