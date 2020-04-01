az keyvault secret show \
  --vault-name mhra-non-prod-02 \
  --name search-index-creator-env \
  --query value \
  --output tsv