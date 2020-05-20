.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-non-prod \
	  --name infrastructure-non-prod-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env
set-env: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
	--vault-name mhra-non-prod \
	--name infrastructure-non-prod-env \
	--file .env

.PHONY: plan
plan: ## Reads from .env and runs terraform plan with those variables
	export $(cat .env .env.overrides 2> /dev/null | xargs) && \
	terraform plan