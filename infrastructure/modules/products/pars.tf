resource "azurerm_storage_account" "pars" {
  name                     = var.pars_namespace
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

resource "azurerm_cdn_profile" "pars" {
  name                = "mhrapars${var.environment}"
  location            = "westeurope"
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "pars" {
  name                = "mhrapars${var.environment}"
  profile_name        = azurerm_cdn_profile.pars.name
  location            = azurerm_cdn_profile.pars.location
  resource_group_name = var.resource_group_name
  origin_host_header  = azurerm_storage_account.pars.primary_web_host
  origin {
    name      = "mhrapars${var.environment}"
    host_name = azurerm_storage_account.pars.primary_web_host
  }
}
