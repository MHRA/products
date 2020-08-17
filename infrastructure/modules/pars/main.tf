resource "azurerm_storage_account" "pars" {
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

resource "azurerm_cdn_endpoint" "pars" {
  name                = var.namespace
  profile_name        = var.cdn_name
  location            = var.cdn_region
  resource_group_name = var.resource_group_name
  origin_host_header  = azurerm_storage_account.pars.primary_web_host
  origin {
    name      = var.namespace
    host_name = azurerm_storage_account.pars.primary_web_host
  }

  delivery_rule {
    name  = "httpredirect"
    order = 1

    request_scheme_condition {
      match_values = [
        "HTTP",
      ]
      negate_condition = false
      operator         = "Equal"
    }

    url_redirect_action {
      protocol      = "Https"
      redirect_type = "Moved"
    }
  }
}
