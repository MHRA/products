provider "azurerm" {
  version = "~> 1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    storage_account_name = "mhranonprodtfstate"
    container_name       = "tfstate"
    key                  = "non-prod.terraform.tfstate"
  }
}

locals {
  namespace = "mhraproductsnonprod"
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION

  tags = {
    environment = var.ENVIRONMENT
  }
}

# website
module "products" {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = azurerm_resource_group.products.name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                       = var.CLIENT_ID
  client_secret                   = var.CLIENT_SECRET
  environment                     = var.ENVIRONMENT
  location                        = var.REGION
  resource_group_name             = azurerm_resource_group.products.name
  vnet_name                       = "aparz-spoke-np-products"
  vnet_cidr                       = "10.5.0.0/16"
  subnet_name                     = "adarz-spoke-products-sn-01"
  subnet_cidr                     = "10.5.65.0/24"
  route_table_name                = "adarz-spoke-rt-products-internal-only"
  route_table_resource_group_name = "asazr-rg-1001"
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = azurerm_resource_group.products.name
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = "doc-index-updater-non-prod"
  resource_group_name = azurerm_resource_group.products.name
}
