app := doc-index-updater
image := mhraproductsnonprodregistry.azurecr.io/products/$(app)
tag := latest

.PHONY: default
default: ## Run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo run

.PHONY: test
test: ## Run tests [TEST=test_name (optional)]
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo test $$TEST && cargo clippy

.PHONY: e2e
e2e: ## Run end to end acceptance tests [TEST=test_name (optional)]
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo test $$TEST -- --ignored

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name doc-index-updater-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name doc-index-updater-build-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env.build

.PHONY: set-env
set-env: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
		--vault-name mhra-dev \
		--name doc-index-updater-env \
		--file .env
	az keyvault secret set \
		--vault-name mhra-dev \
		--name doc-index-updater-build-env \
		--file .env.build

.PHONY: set-sftp-keys
set-sftp-keys: ## Adds pub/private key for localhost to your .ssh dir
	az keyvault secret show \
		--vault-name mhra-dev \
		--name dev-sentinel-public-key \
		--query value \
		--output tsv > ~/.ssh/doc_index_updater.pub && \
	az keyvault secret show \
		--vault-name mhra-dev \
		--name dev-sentinel-private-key \
		--query value \
		--output tsv > ~/.ssh/doc_index_updater && \
	SENTINEL_PRIVATE_KEY_PASSWORD=$$(az keyvault secret show \
		--vault-name mhra-dev  \
		--name dev-sentinel-private-key-password \
		--query value \
		--output tsv) && \
	echo "SENTINEL_PUBLIC_KEY_PATH=\"$${HOME}/.ssh/doc_index_updater.pub\"" >> .env.overrides && \
	echo "SENTINEL_PRIVATE_KEY_PATH=\"$${HOME}/.ssh/doc_index_updater\"" >> .env.overrides && \
	echo "SENTINEL_PRIVATE_KEY_PASSWORD=\"$${SENTINEL_PRIVATE_KEY_PASSWORD}\"" >> .env.overrides

.PHONY: ci-branch
ci-branch: docker-build ## Build branch in CI

.PHONY: ci-master
ci-master: ci-branch docker-push ## Build master in CI

.PHONY: docker-build
docker-build: ## Build and tag Docker image (with search-client create as dependency)
	tar cv Dockerfile ./target/release/doc_index_updater | \
		DOCKER_BUILDKIT=1 docker build - \
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
	docker push $(image):$(tag)

.PHONY: docker-retag
docker-retag: ## Update tag of Docker image
	docker tag ${image}:${tag} ${new_image}:${tag}

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
