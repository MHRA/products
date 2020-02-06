## Workflow orchestration

We are exploring the use of [Zeebe](https://docs.zeebe.io/index.html) for workflow orchestration within the context of a Microservice.

Whilst a Kubernetes cluster may have a single workflow engine installed, individual workflows are context-bound to a particular microservice and are owned and deployed by the service. They may well orchestrate tasks performed by other microservices (e.g. Notifcations or Payments) but we want to have a healthy mix of choreography (between services) and orchestration (within services).

The [./manifests](./manifests) directory contains kubernetes manifests for deployment of a fault-tolerant (3 node) Zeebe cluster to an (at least) 3 node K8s cluster.
