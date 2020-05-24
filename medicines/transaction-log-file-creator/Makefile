app := transaction-log-file-creator

.PHONY: default
default: ## Run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo run

.PHONY: test
test: ## Run tests [TEST=test_name (optional)]
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo test $$TEST && cargo clippy

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name transaction-log-file-creator-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env
set-env: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
		--vault-name mhra-dev \
		--name transaction-log-file-creator-env \
		--file .env

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
