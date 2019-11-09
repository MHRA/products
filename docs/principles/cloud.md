## Cloud

The proposed design fully embraces the cloud strategy and is almost completely agnostic to which cloud provider is used. It does not mandate any proprietary software or platforms.

Cloud providers are much better at managing infrastructure, services and datastores than organisations such as the agency. We want to leverage managed services as much as possible, especially for storing state. We can provision quickly, significantly reduce costs and reliably store data with effectively infinite scale. For example, CosmosDB has an SLA of 99.999% uptime (5-nines), which is the equivalent of about 5 minutes downtime per year. Data is replicated across zones and/or regions. This is much more reliable (and much cheaper) than we could ever hope to achieve if we were managing our own databases.

The architecture described below fully resides in Microsoft Azure. However, it would be identical if it were to be built in AWS or GCP (we would just replace AKS with EKS or GKE, CosmosDB with DynamoDB or Firestore, Blob Storage with S3 or Cloud Storage). This allows us potentially to move to another provider with little additional effort if needed. It also allows us to adopt a multi-cloud strategy, meaning we don’t place all our eggs in one basket.
