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

resource "azurerm_cdn_endpoint" "pars" {
  name                = var.pars_namespace
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = var.resource_group_name
  origin_host_header  = azurerm_storage_account.pars.primary_web_host
  origin {
    name      = var.pars_namespace
    host_name = azurerm_storage_account.pars.primary_web_host
  }
  is_http_allowed = false
}
