provider "azurerm" {
  version = "~> 1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    storage_account_name = "tfstate25382"
    container_name       = "tfstate"
    key                  = "dev.terraform.tfstate"
  }
}

locals {
  namespace = "mhraproductsdevelopment"
}

# website
module "products" {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = "doc-index-updater-dev"
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}
