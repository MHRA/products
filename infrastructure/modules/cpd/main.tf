resource "azurerm_storage_account" "cpd" {
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

resource "azurerm_cdn_profile" "cpd" {
  name                = "mhracpd${var.environment}"
  location            = var.REGION
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

