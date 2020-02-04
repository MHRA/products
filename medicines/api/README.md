# API rust server

This is where the API rust server lives.

## Prerequisites

You should have `rust` installed, ideally `rustup`.
If you're using homebrew, you should be able to install `rustup` by running:

1. `brew install rustup-init`
2. `rustup-init` and following the instructions

You should also have installed:

- [Docker](https://docs.docker.com/install/)
- Kubernetes [`kubectl`](https://kubernetes.io/docs/tasks/tools/install-kubectl/) to manage the Kubernetes cluster.
- Kubernetes [`stern`](https://github.com/wercker/stern)

## Running locally 🦀

1. Navigate to this directory, `/medicines/api`
2. Run `cargo run`
3. Once compiled, open a browser tab and go to http://127.0.0.1:8080/healthz
4. You should see **OK** rendered on the page

To see the graphql explorer, go to http://127.0.0.1:8080/graphiql.

## Running in Docker container 🐳

1. Navigate to this directory, `/medicines/api`
2. Run `docker build . -t api`
3. Run `docker run -p 8080:8000 api`
4. Open the browser and go to `http://localhost:8080/healthz`
5. Yo should see an **Ok** and a server log in your terminal

## Deploy API pod in Kuberbetes cluster ⎈

1. Navigate to this directory, `/medicines/api`
2. Create [Kubernetes Service](https://kubernetes.io/docs/concepts/services-networking/service/)

   ```sh
   kubectl apply -f ./infrastructure/non-prod/service.yml
   ```

3. Deploy [Kubernetes pod](https://kubernetes.io/docs/concepts/workloads/pods/pod/) by applying a deployment

   ```sh
   kubectl apply -f ./infrastructure/non-prod/deployment.yml
   ```

4. Check if the pods are running

   ```sh
   stern api
   ```

5. You should receive a health server log like this

   ```sh
   api-558646c969-9mdxp api [2020-01-20T15:02:57Z INFO  actix_web::middleware::logger] 10.244.1.1:51524 "GET /healthz HTTP/1.1" 200 2 "-" "kube-probe/1.14" 0.000059
   ```
