## Service Mesh

In order to operate a secure, fast and reliable microservices ecosystem, we need a service mesh to route and manage traffic between services, and secure that traffic with authentication (both end-user and service-to-service) and authorization (role-based access control). Additionally, service meshes allow us to observe how services are behaving and can help us test how our services behave when faults are injected.

We are choosing to use Istio as this is the most mature and feature complete implementation available. One to watch is Linkerd 2.0 which is extremely well implemented, but some important features are not yet available.

All this may seem like a lot of overhead to manage a simple service. However, this is a platform that can be used to run all of the Agency’s services (including those yet to be built) and will become increasingly important as functions are carved out of Sentinel prior to its decommissioning.

By starting out with a service mesh, we get to leverage the mesh’s implementations of all the cross-cutting concerns that services have, allowing the services to purely concentrate on business functionality, making them simpler and much easier and cheaper to build.
