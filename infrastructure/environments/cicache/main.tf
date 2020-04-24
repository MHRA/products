provider "azurerm" {
  version = "=2.2.0"
  features {}
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    storage_account_name = "cicachetfstate"
    container_name       = "tfstate"
    key                  = "cicache.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "ci-cache" {
  name     = "ci-cache"
  location = var.REGION
}

resource "azurerm_storage_account" "ci-cache" {
  name                     = "cicache"
  resource_group_name      = azurerm_resource_group.ci-cache.name
  location                 = var.REGION
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "LRS" # locally redundant storage
}

resource "azurerm_storage_container" "ci-cache" {
  name                  = "cicache"
  storage_account_name  = azurerm_storage_account.ci-cache.name
  container_access_type = "blob"
}
