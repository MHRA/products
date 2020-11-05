.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: get-env-non-prod
get-env-non-prod: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-non-prod-02 \
	  --name import-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env-non-prod
set-env-non-prod: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name import-env \
	--file .env

.PHONY: get-env-production
get-env-production: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhraproducts4853 \
	  --name import-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env-production
set-env-production: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
	--vault-name mhraproducts4853 \
	--name import-env \
	--file .env
