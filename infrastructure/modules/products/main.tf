
resource "azurerm_resource_group" "products" {
  name     = var.resource_group_name
  location = var.location

  tags = {
    environment = var.environment
  }
}


resource "azurerm_storage_account" "products" {
  name                     = var.namespace
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = var.environment
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
  name                = var.namespace
  resource_group_name = var.resource_group_name
  location            = var.location
  sku                 = "basic"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_container_registry" "products" {
  name                = "${var.namespace}registry"
  resource_group_name = var.resource_group_name
  location            = var.location
  sku                 = "Standard"
  admin_enabled       = true

  tags = {
    environment = var.environment
  }
}
