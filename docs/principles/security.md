## Security considerations

Our aim is to apply the architecture described above in a way that is secure by default. This means that workloads that are deployed to the platform will automatically operate with the most secure options enabled.

### TLS

This includes ingress traffic using TLS only (with version negotiation up to and including TLS 1.3) and service-to-service communication within the mesh using mutual TLS (mTLS, client and server certificates, with rotation). HTTP ingress on port 80 will be permanently redirected to HTTPS on port 443.

All egress traffic will be denied unless a relevant Istio ServiceEntry (and associated Istio VirtualService) has been defined. When allowed, egress traffic will be HTTPS only.

### Secrets

A [Bitnami Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets) controller running in the cluster will decrypt Kubernetes secrets that are stored encrypted in the Github repository alongside the other application manifests. The private key for the controller will be stored in [Azure Key Vault](https://github.com/bitnami-labs/sealed-secrets). There will be one key for non-production instances and separate keys for each production instance.

Using encrypted secrets works well with the “Everything as code” principle that we discussed above. It means that passwords and keys for upstream services can be versioned and controlled safely alongside everything else, knowing that they can only be read on the target cluster once they have been decrypted by the controller.

## Authentication

This MVP phase does not require origin authentication (e.g. a JWT bearer token that the user has obtained from an identity provider).

However, service-to-service traffic (including from the ingress controller) will have transport authentication provided by enforcing mTLS and leveraging [Istio Secure Naming](https://istio.io/docs/concepts/security/).

## Authorisation

Kubernetes Role-based Access Control (RBAC) will be used to ensure services have the relevant access to the Kubernetes API.
There is no need in this phase to use [Istio RBAC](https://istio.io/docs/reference/config/authorization/istio.rbac.v1alpha1/) as authentication is not currently required in order to search for and view the SPC/PIL/PAR data.

## Data security

There is no Personally Identifiable Information (PII) of any kind involved in this phase of the solution design. All data is freely available to anyone and therefore does not need encryption in transit or at rest (although in-transit protection will be provided by enforcing TLS throughout).

## Vulnerabilities and attack vectors

Software versions will be kept up to date in order to ensure that security fixes for Common Vulnerabilities and Exposures (CVE) are consumed as soon as possible after becoming available. This includes Istio, Kubernetes and Linux versions (both within containers and on nodes). Nodes in the cluster will be refreshed regularly, and new nodes will be created from up to date images. Containers will be built from up to date base images as they pass through the CI/CD pipeline. We will need to evolve policies for ensuring that this happens and is auditable (again the “everything as code” principle helps us with audit and traceability).

As we build web applications, services and APIs that run on the cluster, we will need to ensure that (at least) the [OWASP top ten](https://blog.sucuri.net/2018/10/owasp-top-10-security-risks-part-i.html) security risks are considered. Many do not apply to this design (e.g. SQL injection attacks because there is no SQL or direct database access, broken authentication as there is no authentication, broken access control as authorisation is not required, XML external entities as there is no XML), whilst others do (e.g. security misconfigurations, cross-site scripting, insufficient logging etc). Where relevant, we will use appropriate techniques (e.g. by using React with [additional mitigations](https://stackoverflow.com/questions/33644499/what-does-it-mean-when-they-say-react-is-xss-protected) for XSS, and [CSRF tokens](https://portswigger.net/web-security/csrf/tokens) in forms, exhaustive access logging, etc) to mitigate these attack vectors.
