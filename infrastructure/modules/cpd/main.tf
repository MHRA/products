resource "azurerm_storage_account" "cpd" {
  name                     = var.namespace
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  static_website {
    error_404_document = "404.html"
    index_document     = "index.html"
  }

  tags = {
    environment = var.environment
  }
}

resource "azurerm_storage_container" "cpd_website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.cpd.name
  container_access_type = "container"
}

resource "azurerm_cdn_profile" "cpd" {
  name                = "mhracpd${var.environment}"
  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "cpd" {
  name                = "mhracpd${var.environment}"
  profile_name        = azurerm_cdn_profile.cpd.name
  location            = azurerm_cdn_profile.cpd.location
  resource_group_name = var.resource_group_name
  origin_host_header  = azurerm_storage_account.cpd.primary_web_host
  origin {
    name      = "mhracpd${var.environment}"
    host_name = azurerm_storage_account.cpd.primary_web_host
  }
}
