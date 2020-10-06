.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: get-env
get-env: ## Gets the non-prod environment variables from azure keyvault into .env file
	az keyvault secret show \
	--vault-name mhra-non-prod-02 \
	--name pars-env \
	--query value \
	--output tsv > .env

.PHONY: set-env
set-env: ## Replaces the keyvault value for non-prod with your current .env file
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name pars-env \
	--file .env

.PHONY: e2e
e2e: ## Runs end-to-end tests, including dev server. Headless.
	yarn test-e2e

.PHONY: e2e-headed
e2e-headed: ## Runs end-to-end tests, including dev server. Headed.
	yarn test-e2e-headed
