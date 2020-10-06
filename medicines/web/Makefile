.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-non-prod-02 \
	  --name web-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env
set-env: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name web-env \
	--file .env
