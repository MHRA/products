app := medicines-api
registry := mhraproductsnonprodregistry
image := $(registry).azurecr.io/products/$(app)
tag := latest

.PHONY: default
default: ## Run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo run

.PHONY: test
test: ## Run tests [TEST=test_name (optional)]
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo test $$TEST && cargo clippy

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name api-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name api-build-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env.build

.PHONY: set-env
set-env: ## Takes your current .env and .env.build files and replaces the keyvault value with the contents of the files
	az keyvault secret set \
		--vault-name mhra-dev \
		--name api-env \
		--file .env
	az keyvault secret set \
		--vault-name mhra-dev \
		--name api-build-env \
		--file .env.build

.PHONY: ci-branch
ci-branch: docker-build ## Build branch in CI

.PHONY: ci-master
ci-master: ci-branch docker-push ## Build master in CI

.PHONY: docker-build
docker-build: ## Build and tag Docker image (with search-client create as dependency)
	export $$(cat .env.build 2> /dev/null | xargs) && \
	tar cv --exclude=target Dockerfile -C .. ./api ./search-client | DOCKER_BUILDKIT=1 docker build - \
		--build-arg SCCACHE_AZURE_CONNECTION_STRING \
		--progress plain \
		--tag $(app) \
		--tag $(image) \
		--tag $(image):$(tag)

.PHONY: docker-run
docker-run: ## Run Docker image
	docker run -it --env-file=.env $(image)

.PHONY: docker-pull
docker-pull: ## Pull Docker image
	docker pull $(image):$(tag)

.PHONY: docker-push
docker-push: ## Push Docker image
	docker push $(image)
	docker push $(image):$(tag)

.PHONY: docker-retag
docker-retag: ## Update tag of Docker image
	docker tag ${image}:${tag} ${new_image}:${tag}

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
