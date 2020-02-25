## Service Mesh

In order to operate a secure, fast and reliable microservices ecosystem, we need a [service mesh](https://www.redhat.com/en/topics/microservices/what-is-a-service-mesh) to route and manage traffic between services, and secure that traffic with authentication (both end-user and service-to-service) and authorization (role-based access control). Additionally, service meshes allow us to observe how services are behaving and can help us test how our services behave when faults are injected.

We are choosing to use [Istio](https://istio.io/) as this is the most mature and feature complete implementation available. One to watch is [Linkerd 2.0](https://linkerd.io/2/overview/) which is extremely well implemented, but some important features are not yet available.
