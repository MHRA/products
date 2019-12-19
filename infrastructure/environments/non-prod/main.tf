provider "azurerm" {
  version = "=1.38.0"
}

terraform {
  required_version = "0.12.18"

  backend "azurerm" {
    resource_group_name = "tfstate"
    key                 = "non-prod.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION

  tags = {
    environment = "non-prod"
  }
}

resource "azurerm_storage_account" "products" {
  name                     = "mhraproductsnonprod"
  resource_group_name      = azurerm_resource_group.products.name
  location                 = azurerm_resource_group.products.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = "non-prod"
  }
}

resource "azurerm_storage_container" "products_website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "container"
}

resource "azurerm_storage_container" "docs" {
  name                  = "docs"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "blob"
}

# waiting for this to be resolved: https://github.com/terraform-providers/terraform-provider-azurerm/issues/1903
# (which is imminent), but in the meantime ...
module "products_staticweb" {
  source               = "git@github.com:StefanSchoof/terraform-azurerm-static-website.git"
  storage_account_name = azurerm_storage_account.products.name
}

resource "azurerm_search_service" "search" {
  name                = "mhraproductsnonprod"
  resource_group_name = azurerm_resource_group.products.name
  location            = azurerm_resource_group.products.location
  sku                 = "basic"

  tags = {
    environment = "non-prod"
  }
}

resource "azurerm_storage_account" "cpd" {
  name                     = "mhracpdnonprod"
  resource_group_name      = azurerm_resource_group.products.name
  location                 = azurerm_resource_group.products.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = "non-prod"
  }
}

resource "azurerm_storage_container" "cpd_website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.cpd.name
  container_access_type = "container"
}

# waiting for this to be resolved: https://github.com/terraform-providers/terraform-provider-azurerm/issues/1903
# (which is imminent), but in the meantime ...
module "cpd_staticweb" {
  source               = "git@github.com:StefanSchoof/terraform-azurerm-static-website.git"
  storage_account_name = azurerm_storage_account.cpd.name
}


resource "azurerm_cdn_profile" "products" {
  name                = "mhraproductsnonprod"
  location            = "westeurope"
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
