## Security considerations

We aim to design systems in a way that they are secure by default. This means that workloads that are deployed to a platform will automatically operate with the most secure options enabled.

### TLS

This includes ingress traffic using TLS only (with version negotiation up to and including TLS 1.3) and service-to-service communication within the mesh using mutual TLS (mTLS, client and server certificates, with rotation). HTTP ingress on port 80 will be permanently redirected to HTTPS on port 443.

All egress traffic will be denied unless a relevant Istio ServiceEntry (and associated Istio VirtualService) has been defined. When allowed, egress traffic will be HTTPS only.

### Secrets

A [Bitnami Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets) controller running in the cluster will decrypt Kubernetes secrets that are stored encrypted in the Github repository alongside the other application manifests. The private key for the controller will be stored in [Azure Key Vault](https://github.com/bitnami-labs/sealed-secrets). There will be one key for non-production instances and separate keys for each production instance.

Using encrypted secrets works well with the “Everything as code” principle that we discussed above. It means that passwords and keys for upstream services can be versioned and controlled safely alongside everything else, knowing that they can only be read on the target cluster once they have been decrypted by the controller.

## Authentication

Origin authentication can be ensured by using e.g. a JWT bearer token that the user has obtained from an identity provider.

Service-to-service traffic (including from the ingress controller) should have transport authentication provided by enforcing mTLS and leveraging [Istio Secure Naming](https://istio.io/docs/concepts/security/).

## Authorisation

Kubernetes Role-based Access Control (RBAC) is used to ensure services have the relevant access to the Kubernetes API.
[Istio RBAC](https://istio.io/docs/reference/config/authorization/istio.rbac.v1alpha1/) can be used, where necessary.

## Data security

Any Personally Identifiable Information (PII) should be encrypted in transit (by enforcing TLS throughout) and at rest.

## Vulnerabilities and attack vectors

Software versions should be kept up to date in order to ensure that security fixes for Common Vulnerabilities and Exposures (CVE) are consumed as soon as possible after becoming available. Nodes in clusters should be refreshed regularly, and new nodes created from up to date images. Containers should be built from up-to-date base images as they pass through the CI/CD pipeline. We will need to evolve policies for ensuring that this happens and is auditable (again the “everything as code” principle helps us with audit and traceability).
