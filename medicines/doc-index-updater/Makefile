.PHONY: default
default:
	export $$(cat .env | xargs) && cargo run

.PHONY: docker
docker:
	docker build \
		--tag doc-index-updater \
		--tag mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater \
		.

.PHONY: docker-run
docker-run:
	docker run -it --env-file=.env mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater

.PHONY: docker-pull
docker-pull:
	docker pull mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater

.PHONY: docker-push
docker-push:
	docker push mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater
