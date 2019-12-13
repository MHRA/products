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

resource "azurerm_resource_group" "rg" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION
}

resource "azurerm_storage_account" "products" {
  name                     = "mhraproductsnonprod"
  resource_group_name      = azurerm_resource_group.rg.name
  location                 = azurerm_resource_group.rg.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"
}

resource "azurerm_storage_container" "website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "container"
}
