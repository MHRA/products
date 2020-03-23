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

  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
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


resource "azurerm_cdn_profile" "products" {
  name                = "mhraproductsnonprod"
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = "mhraproductsnonprod"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = azurerm_resource_group.products.name
  origin_host_header  = azurerm_storage_account.products.primary_web_host
  origin {
    name      = "mhraproductsnonprod"
    host_name = azurerm_storage_account.products.primary_web_host
  }
}

resource "azurerm_cdn_endpoint" "cpd" {
  name                = "mhracpdnonprod"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = azurerm_resource_group.products.name
  origin_host_header  = azurerm_storage_account.cpd.primary_web_host
  origin {
    name      = "mhracpdnonprod"
    host_name = azurerm_storage_account.cpd.primary_web_host
  }
}
