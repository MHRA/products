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

.PHONY: pull
pull:
	docker login mhraproductsdevelopmentregistry.azurecr.io \
		-u 00000000-0000-0000-0000-000000000000 \
		-p $${ACR_ACCESS_TOKEN} && \
	docker pull mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater

.PHONY: push
push:
	docker login mhraproductsdevelopmentregistry.azurecr.io \
		-u 00000000-0000-0000-0000-000000000000 \
		-p $${ACR_ACCESS_TOKEN} && \
	docker push mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater