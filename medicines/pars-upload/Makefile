.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: get-env
get-env: ## Gets the environment variables from azure keyvault into .env file
	az keyvault secret show \
	  --vault-name mhra-dev \
	  --name pars-env \
	  --query value \
	  --output tsv | sort | sed '/^$$/d' > .env

.PHONY: set-env
set-env: ## Takes your current .env file and replaces the keyvault value with the contents of the file
	az keyvault secret set \
	--vault-name mhra-dev \
	--name pars-env \
	--file .env

.PHONY: e2e
e2e: ## Runs end-to-end tests, including dev and mock servers, and cleans up afterwards.
	yarn mock & wait-on http://localhost:8000; \
	mock_server_pid = $$!; \
	NEXT_PUBLIC_DISABLE_AUTH=true yarn dev & wait-on http://localhost:3000; \
	dev_server_pid=$$!; \
	test_exit_code=$$?; \
	yarn run cypress run; \
	kill $${dev_server_pid}; \
	kill $${mock_server_pid}; \
	exit $${test_exit_code}
