provider "azurerm" {
  skip_provider_registration = true
  version                    = "~>1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name = "tfstate"
    key                 = "prod.terraform.tfstate"
  }
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
  resource_group_name = azurerm_resource_group.products.name
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  name                = "doc-index-updater-prod"
}

resource "azurerm_cdn_profile" "products" {
  name                = "mhraproductsprod"
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = "mhraproductsprod"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = azurerm_resource_group.products.name
  origin_host_header  = azurerm_storage_account.products.primary_web_host
  origin {
    name      = "mhraproductsprod"
    host_name = azurerm_storage_account.products.primary_web_host
  }
}

resource "azurerm_cdn_endpoint" "cpd" {
  name                = "mhracpdprod"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = azurerm_resource_group.products.name
  origin_host_header  = azurerm_storage_account.cpd.primary_web_host
  origin {
    name      = "mhracpdprod"
    host_name = azurerm_storage_account.cpd.primary_web_host
  }
}
