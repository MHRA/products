# products.mhra.gov.uk Website Infrastucture

Instructions on getting the website set up.
As steps are completed, the developer completing those steps is responsible for updating this document.
This document should become a guide for setting up the website infrastructure on a new environment and for connecting to the existing environment.

## Docker

If you want to use Docker to run the website locally, the good news is that you can!

```sh
docker-compose up
```

It's as simple as that.

### Building, pushing, running from registry

Provisioning the website as a docker image, ready for use in Azure Kubernetes cluster (see [_To-do Steps_](#to-do-steps)), is a little more complicated that running it locally. For all of the below steps, you'll want to be in `/medicines/web`.

1. Build

   ```sh
   docker-compose build
   ```

   This will build the docker image locally, using the `docker-compose.yml`.
   It runs the `Dockerfile`, and tags the built image as `web:latest`.

2. Login

   ```sh
   az acr login --name mhraproductsdevregistry
   ```

   This should work, provided you're logged into the right Azure account.

3. Tag and push

   ```sh
   docker tag web mhraproductsdevregistry.azurecr.io/products/web
   docker push mhraproductsdevregistry.azurecr.io/products/web
   ```

   The first line here lets docker know that the origin for the pushed image should be your local `web` image.
   The second line does the pushing.

All being well, you'll now be able to run the pushed image, locally. Test it out:

```sh
docker run -it --rm -p 3000:3000 mhraproductsdevregistry.azurecr.io/products/web
```

Head to [http://localhost:3000][localhost] in your browser to see a locally hosted version of the docker image you've pushed.

If you're unfamiliar with the command above, it's worth [reading the `docker run` documentation][docker run].
In short, the `-it` parameters above allow command-line input (which means shortcuts like <kbd>Ctrl</kbd>+<kbd>C</kbd> will work),
and `--rm` instructs docker to cleanup the container automatically when you're finished.

## To-do Steps

This infrastructure is a work-in-progress.
The steps below are a guide to get to the desired infrastructure.

1. Create Kubernetes pod

   First, you'll need YAML specifying the pod's configuration.
   This should refer to the docker image in the container registry.

   Save it at `/medicines/web/infrastructure/<environment>/deployment.yml`.
   After that, creating the pod in kubernetes is easy, provided you've already followed the [kubernetes setup instructions][repo kubernetes setup].

   ```sh
   kubectl apply -f ./infrastructure/<environment>/deployment.yml
   ```

   Update this documentation for how to use the Kubernetes infrastructure.

2. Server-side rendering

   Now that the site is running in a docker container, it's time to switch to [server-side rendering][nextjs ssr].

   This will require use of [`getInitialProps`][nextjs getinitialprops], so that some data is available immediately, and first meaningful paint is faster!

   Update this documentation with information on how we've used server-side rendering, as a guide for additional development later.

[localhost]: http://localhost:3000 'Your friendly neighbourhood localhost'
[docker run]: https://docs.docker.com/engine/reference/run/ 'docker run | Docker Documentation'
[repo kubernetes setup]: ../../../infrastructure/docs/kubernetes.md 'MHRA Medicines microservices - Kubernetes'
[nextjs ssr]: https://nextjs.org/docs/basic-features/pages#server-side-rendering 'Pages - Documentation | Nextjs'
[nextjs getinitialprops]: https://nextjs.org/docs/api-reference/data-fetching/getInitialProps 'getInitialProps - Documentation | Nextjs'
