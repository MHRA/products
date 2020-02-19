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
  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  products_namespace  = "mhraproductsdev"
  cpd_name_space      = "mhracpddev"
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# website
module "products" {
  source = "../../modules/products"

  environment         = local.environment
  location            = local.location
  namespace           = local.products_namespace
  resource_group_name = local.resource_group_name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id           = local.client_id
  client_secret       = local.client_secret
  environment         = local.environment
  location            = local.location
  resource_group_name = local.resource_group_name
}

# CPD
module cpd {
  source = "../../modules/cpd"

  resource_group_name = local.resource_group_name
  location            = local.location
  environment         = local.environment
  namespace           = local.cpd_name_space
}




