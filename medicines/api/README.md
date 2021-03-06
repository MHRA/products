# API rust server

![medicines-api](https://github.com/MHRA/products/workflows/medicines-api-master/badge.svg)

This is where the code for the [Medicines API](https://medicines.api.mhra.gov.uk) lives.

## Prerequisites

You should have `rust` installed, ideally `rustup`.
If you're using homebrew, you should be able to install `rustup` by running:

1. `brew install rustup-init`
2. `rustup-init` and following the instructions

You may be able to install `rustup` through your chosen package manager, if you use one.

Otherwise, visit [rust-lang's `rustup` installation guide][rustup install], which has instructions for your operating system.
For installing

You should also have installed:

- [Docker][docker install]
- [Kubernetes `kubectl`][kubernetes install] to manage the Kubernetes cluster.
- [Kubernetes `stern`][stern]

## Running locally 🦀

1. Navigate to this directory (`/medicines/api`)
2. Run `make get-env` to get a copy of [environment variables](../../docs/principles/config.md)
3. Run `make` to build and run the server
4. Once compiled, open a browser tab and go to http://127.0.0.1:8000/healthz
5. You should see **OK** rendered on the page

To see the GraphQL explorer, go to http://127.0.0.1:8000.

## Running in Docker container 🐳

1. Navigate to this directory (`/medicines/api`)
2. Run `DOCKER_BUILDKIT=1 docker build . -t api`
3. Run `docker run -p 8080:8000 --env-file .env api`
4. Open the browser and go to `http://localhost:8080/healthz`
5. You should see an **Ok** and a server log in your terminal

To see the GraphQL explorer, go to http://127.0.0.1:8080.

## Deploy API pod in Kubernetes cluster ⎈

1. Navigate to this directory (`/medicines/api`)
2. Source environmental variables from the corresponding environment

   ```sh
   source ./infrastructure/non-prod/.env
   ```

3. Create [Kubernetes secret][kubernetes secret] for API Pod to consume

   ```sh
   kubectl create secret generic azure-search-secrets \
   --from-literal=AZURE_SEARCH_KEY="$AZURE_SEARCH_KEY" \
   --from-literal=AZURE_SEARCH_INDEX="$AZURE_SEARCH_INDEX" \
   --from-literal=AZURE_SEARCH_SERVICE="$AZURE_SEARCH_SERVICE" \
   ```

4. Create [Kubernetes Service][kubernetes service]

   ```sh
   kubectl apply -f ./infrastructure/non-prod/service.yml
   ```

5. Deploy [Kubernetes pod][kubernetes pod] by applying a deployment

   ```sh
   kubectl apply -f ./infrastructure/non-prod/deployment.yml
   ```

6. Check if the pods are running

   ```sh
   stern api
   ```

7. You should receive a health server log like this

   ```sh
   api-558646c969-9mdxp api [2020-01-20T15:02:57Z INFO  actix_web::middleware::logger] 10.244.1.1:51524 "GET /healthz HTTP/1.1" 200 2 "-" "kube-probe/1.14" 0.000059
   ```

[rustup install]: https://www.rust-lang.org/tools/install "Install Rust - Rust Programming Language"
[docker install]: https://docs.docker.com/install/ "Install Docker"
[kubernetes install]: https://kubernetes.io/docs/tasks/tools/install-kubectl/ "Install Kubernetes"
[stern]: https://github.com/wercker/stern "Stern - GitHub"
[kubernetes service]: https://kubernetes.io/docs/concepts/services-networking/service/ "Service - Kubernetes Documentation"
[kubernetes pod]: https://kubernetes.io/docs/concepts/workloads/pods/pod/ "Pod - Kubernetes Documentation"
[kubernetes secret]: https://kubernetes.io/docs/concepts/configuration/secret/ "Secret - Kubernetes Documentation"

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `medicinesapi.vX.X.X` (e.g. `medicinesapi.v1.3.0`) at the commit you want to release, incrementing as required from the most recent version. The `medicines-api-release` workflow will then pull the image from the `non-prod` container registry (created as part of the master branch workflow), push the image to the `production` container registry in Azure and then update the commit SHA in the production overlay in deployments repo. ArgoCD will identify this change, pull the linked image from the container registry and deploy it to the cluster.
