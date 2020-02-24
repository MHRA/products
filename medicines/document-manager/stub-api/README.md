# Stub API

## Purpose

The stub API is built to unblock integration testing for Sentinel, **and has
no practical functionality**. It does not interact with the Sentinel server
or Azure Blob storage in any way, it simply uses in-memory storage to mock
the actual API functionality.

The intended usage pattern is as follows:

- Send a DELETE or POST request for a document;
- Continue to poll the GET endpoint for that document until the status has
  updated.

In the background, the real API will asynchronously process the request and
update the status of the document as it progresses.

## Running

```
$ docker build -t stub-document-manager-api
$ docker run -p 8080:8080 -it --rm stub-document-manager-api
```

The webserver listens on http://0.0.0.0:8080 by default.

## Deploying to Kubernetes

First, you need to install the `stable/nginx-ingress` and `cert-manager` packages from Helm.

```
$ export YOUR_IP_ADDRESS=1.2.3.4

$ helm repo add stable https://kubernetes-charts.storage.googleapis.com/
$ helm repo add jetstack https://charts.jetstack.io
$ helm repo update

$ helm install api-stub stable/nginx-ingress \
  --set controller.replicaCount=2 \
  --set controller.nodeSelector."beta\.kubernetes\.io/os"=linux \
  --set defaultBackend.nodeSelector."beta\.kubernetes\.io/os"=linux \
  --set controller.service.loadBalancerIP=$YOUR_IP_ADDRESS \
  --set controller.service.externalTrafficPolicy=Local

$ kubectl apply --validate=false -f https://raw.githubusercontent.com/jetstack/cert-manager/release-0.12/deploy/manifests/00-crds.yaml
$ helm install cert-manager jetstack/cert-manager
```

Then you need to build the Docker image and push it to your container registry:

```bash
$ export STUB_IMAGE=container-registry.yourdomain.com/stub-api:1.0.1

$ docker build -t $STUB_IMAGE .
$ docker push $STUB_IMAGE
```

There are Kubernetes manifests in the `manifests` directory. You can apply these with `kubectl` as usual:

```bash
$ export PUBLIC_URL=yourdomain.com
$ export SSL_EMAIL=you@yourdomain.com
$ export STUB_USERNAME=basic_auth_username
$ export STUB_PASSWORD=basic_auth_password

$ envsubst < manifests/cluster-issuer.yaml | kubectl apply -f -
$ envsubst < manifests/cert.yaml | kubectl apply -f -
$ envsubst < manifests/ingress.yaml | kubectl apply -f -
$ envsubst < manifests/deployment.yaml | kubectl apply -f -
$ envsubst < manifests/svc.yaml | kubectl apply -f -
```

## Authorization

The stub API implements [HTTP basic authentication](https://en.wikipedia.org/wiki/Basic_access_authentication),
using the credentials set in the previous step. Any request without a valid `Authorization` header will receive
a 401 response.

## Endpoints

### GET /jobs/:job_id

Returns the status of a job specified by :job_id.

Sample response:

```xml
<document>
    <document_id>con33333333</document_id>
    <job_id>c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_id>
    <job_uri>https://example.com/jobs/c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_uri>
    <status>done</status>
    <type>delete</type>
</document>
```

### DELETE /documents/:document

Sends a delete request for :document. Sample documents are con10101010,
con20202020, con30303030, con40404040, and con50505050.

A 202 with a job_id will be returned on success, and a 404 if
the requested document does not exist.

Sample response:

```xml
<document>
    <document_id>con33333333</document_id>
    <job_id>c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_id>
    <job_uri>https://example.com/jobs/c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_uri>
    <status>accepted</status>
    <type>delete</type>
</document>
```

### POST /documents/

This expects an XML body shaped like the following:

```xml
<document>
  <id>con33333333</id>
  <name>Name of an SPC</name>
  <type>SPC</type>
  <author>theauthor</author>
  <products>
    <product>Generic Statin</product>
  </products>
  <keywords>
    <keyword>heart disease</keyword>
    <keyword>statin</keyword>
  </keywords>
  <pl_number>PL 12345/0010-0001</pl_number>
  <active_substances>
    <active_substance>statin</active_substance>
  </active_substances>
  <file_source>sentinel</file_source>
  <file_path>/docs/spc/con33333333.pdf</file_path>
</document>
```

It will return a 202 with job_id (and the document will have a
status) if the shape is correct, a 422 if there are missing
required fields, and a 409 if the ID already exists and is not
in the deleted state.

Sample response:

```xml
<document>
    <document_id>con33333333</document_id>
    <job_id>c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_id>
    <job_uri>https://example.com/jobs/c2c0e7db-21e5-46dd-aea8-5c842f195ba2</job_uri>
    <status>accepted</status>
    <type>check-in</type>
</document>
```
