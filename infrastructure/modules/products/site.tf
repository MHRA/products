resource "azurerm_storage_account" "products" {
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

resource "azurerm_storage_container" "docs" {
  name                  = "docs"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "blob"
}

resource "azurerm_storage_container" "bmgf-docs" {
  name                  = "bmgf-docs"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "blob"
}

resource "azurerm_storage_container" "temporary-docs" {
  name                  = "temporary-docs"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "blob"
}

resource "azurerm_storage_management_policy" "products" {
  storage_account_id = azurerm_storage_account.products.id

  rule {
    name    = "removeTemporaryBlobs"
    enabled = true
    filters {
      prefix_match = ["temporary-docs"]
      blob_types   = ["blockBlob"]
    }
    actions {
      base_blob {
        tier_to_cool_after_days_since_modification_greater_than    = 1
        tier_to_archive_after_days_since_modification_greater_than = 2
        delete_after_days_since_modification_greater_than          = 5
      }
      snapshot {
        delete_after_days_since_creation_greater_than = 5
      }
    }
  }
}

resource "azurerm_cdn_profile" "products" {
  name                = var.namespace
  location            = var.cdn_region
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = var.namespace
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = var.resource_group_name
  origin_host_header  = azurerm_storage_account.products.primary_web_host

  origin {
    name      = var.namespace
    host_name = azurerm_storage_account.products.primary_web_host
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
