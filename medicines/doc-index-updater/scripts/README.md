# Load testing scripts

[api.scala Gatling script](api.scala)

To use this script, download [gatling-charts-highcharts-bundle](https://gatling.io/open-source/start-testing/) and place the above scala file in `user-files/simulations/computerdatabase/`

and then from the `bin` folder run `sh ./galtling.sh` and follow the onscreen prompts.

# Functional testing scripts

[delete file (via localhost instance of doc-index-updater)](delete.sh)
Setup environment vars as per the [doc-index-updater README](../README.md) and then run 
```
sh ./delete.sh <doc id here>
```
This will call the api on http://localhost:8000 and check the job status to see if it succeeds or not
