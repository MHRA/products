app := update-metadata

.PHONY: default
default: ## Run locally
	export $$(cat .env .env.overrides 2> /dev/null | xargs) && cargo run
