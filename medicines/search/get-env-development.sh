az keyvault secret show \
  --vault-name mhra-non-prod \
  --name search-index-creator-env \
  --query value \
  --output tsv
