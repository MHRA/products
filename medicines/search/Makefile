.PHONY: get-env-dev
get-env-dev: ## run locally
	az keyvault secret show \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-development-env \
	--query value \
	--output tsv > .env

.PHONY: set-env-dev
set-env-dev: ## run locally
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-development-env \
	--file .env

.PHONY: get-env-non-prod
get-env-non-prod: ## run locally
	az keyvault secret show \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-non-prod-env \
	--query value \
	--output tsv > .env

.PHONY: set-env-non-prod
set-env-non-prod: ## run locally
	az keyvault secret set \
	--vault-name mhra-non-prod-02 \
	--name search-index-creator-non-prod-env \
	--file .env