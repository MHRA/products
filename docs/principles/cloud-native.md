## Docker, Kubernetes and Cloud Native

True portability, though, comes from using industry standard Docker containers for packaging and deploying our workloads (web apps, microservices, legacy apps etc). They can be deployed to any orchestrator (Kubernetes, DC/OS, Mesos, Docker Swarm), but 2 years ago Kubernetes won the battle and is now used almost universally at organisations across the world. All the major cloud providers have managed Kubernetes offerings (AWS EKS, GCP GKE and Azure AKS), which are free to use (you only pay for the VMs that form the nodes in the cluster).

All the cloud providers also have serverless offerings (AWS Lambda, GCP Cloud Functions and Azure functions). This is an even higher level of abstraction where you deploy just functions and pay per second of runtime. Serverless is becoming a viable alternative, but whilst it is cheap for intermittent workloads, it can be expensive for continuous workloads and we want to have our microservices always running. Serverless cold start times are still too long, adversely affecting user experience and possibly making Serverless an unsuitable option for this design.

Inside the Docker containers are Cloud Native workloads. There is an excellent article on the New Stack which describes what it means to be Cloud Native, but basically cloud native services are designed to be lightweight, run in containers, are 12-factor, and can be built, managed and deployed using DevOps practices.

This design incorporates stateless cloud native microservices, running in Docker containers, orchestrated by Kubernetes provided by Azure AKS, consuming data stored in managed services such as Azure CosmosDB, Azure Blob Storage, and Azure Search, monitored by Azure Monitor, and reporting access activity to Azure Activity Log.
