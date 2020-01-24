# products.mhra.gov.uk Website Infrastucture

Instructions on getting the website set up.
As steps are completed, the developer completing those steps is responsible for updating this document.
This document should become a guide for setting up the website infrastructure on a new environment and for connecting to the existing environment.

## Steps

1. Tidy up the `/medicines/web` folder by moving appropriate files into `./src`.

2. Docker time!

   Create a `Dockerfile` in `/medicines/web` which will run `yarn dev`, serving up the Nextjs website.
   Verify that this is working locally by running the docker image, and connecting to the website.

   You will need to push the docker image to the container registry for each of our environments.

3. Create Kubernetes pod

   First, you'll need YAML specifying the pod's configuration.
   This should refer to the docker image in the container registry.

   Save it at `/medicines/web/infrastructure/<environment>/deployment.yml`.
   After that, creating the pod in kubernetes is easy, provided you've already followed the [kubernetes setup instructions][repo kubernetes setup].

   ```sh
   kubectl apply -f ./infrastructure/<environment>/deployment.yml
   ```

4. Server-side rendering

   Now that the site is running in a docker container, it's time to switch to [server-side rendering][nextjs ssr].

[repo kubernetes setup]: ../../../infrastructure/docs/kubernetes.md "MHRA Medicines microservices - Kubernetes
[nextjs ssr]: https://nextjs.org/docs/basic-features/pages#server-side-rendering 'Pages - Documentation | Nextjs'
