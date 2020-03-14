app := doc-index-updater
image := mhraproductsdevelopmentregistry.azurecr.io/products/$(app)
tag := latest

.PHONY: default
default: ## run locally
	export $$(cat .env | xargs) && cargo run

.PHONY: ci
ci: docker-build docker-push ## Build in CI

.PHONY: docker-build
docker-build: ## Build and tag Docker image
	docker build \
		--progress plain \
		--tag $(app) \
		--tag $(image):latest \
		--tag $(image):$(tag) \
		--tag $(image) \
		.

.PHONY: docker-run
docker-run: ## Run Docker image
	docker run -it --env-file=.env $(image)

.PHONY: docker-pull
docker-pull: ## Pull Docker image
	docker pull $(image)

.PHONY: docker-push
docker-push: ## Push Docker image
	docker push $(image):latest
	docker push $(image):$(tag)

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'