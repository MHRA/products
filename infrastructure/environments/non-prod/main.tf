provider "azurerm" {
  version = "~> 1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    container_name       = "tfstate"
    storage_account_name = "tfstate4338"
    key                  = "non-prod.terraform.tfstate"
  }
}

locals {
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  location            = var.REGION
  environment         = var.ENVIROMENT
  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
}

# website
module "products" {
  source              = "../../modules/products"
  resource_group_name = local.resource_group_name
  location            = local.location
  environment         = local.environment
}

# AKS
module cluster {
  source              = "../../modules/cluster"
  resource_group_name = local.resource_group_name
  location            = local.location
  environment         = local.environment
  client_id           = local.client_id
  client_secret       = local.client_secret
}

# CPD
module cpd {
  source              = "../../modules/cpd"
  resource_group_name = local.resource_group_name
  location            = local.location
  environment         = local.environment
}




