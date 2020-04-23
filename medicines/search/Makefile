.PHONY: get-env-dev
get-env-dev: ## Gets the development environment variables from azure keyvault into .env file
	az keyvault secret show \
	--vault-name mhra-dev \
	--name search-env \
	--query value \
	--output tsv > .env

.PHONY: set-env-dev
set-env-dev: ## Replaces the keyvault value for development with your current .env file
	az keyvault secret set \
	--vault-name mhra-dev \
	--name search-env \
	--file .env

.PHONY: get-env-non-prod
get-env-non-prod: ## Gets the non-prod environment variables from azure keyvault into .env file
	az keyvault secret show \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-non-prod-env \
	--query value \
	--output tsv > .env

.PHONY: set-env-non-prod
set-env-non-prod: ## Replaces the keyvault value for non-prod with your current .env file
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-non-prod-env \
	--file .env

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
