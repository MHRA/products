app := doc-index-updater
image := mhraproductsnonprodregistry.azurecr.io/products/$(app)
tag := latest

.PHONY: default
default: ## run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo run

.PHONY: test
test: ## run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo test

.PHONY: get-env
get-env: ## run locally
	az keyvault secret show \
	  --vault-name mhra-non-prod-02 \
	  --name doc-index-updater-env \
	  --query value \
	  --output tsv > .env

.PHONY: set-env
set-env: ## run locally
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name doc-index-updater-env \
	--file .env

.PHONY: ci-branch
ci-branch: docker-build ## Build branch in CI

.PHONY: ci-master
ci-master: ci-branch docker-push ## Build master in CI

.PHONY: docker-build
docker-build: ## Build and tag Docker image
	docker build \
		--progress plain \
		--tag $(app) \
		--tag $(image) \
		--tag $(image):$(tag) \
		.

.PHONY: docker-run
docker-run: ## Run Docker image
	docker run -it --env-file=.env $(image)

.PHONY: docker-pull
docker-pull: ## Pull Docker image
	docker pull $(image)

.PHONY: docker-push
docker-push: ## Push Docker image
	docker push $(image)
	docker push $(image):$(tag)

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
