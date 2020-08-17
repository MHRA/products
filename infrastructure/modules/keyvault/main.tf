data "azurerm_client_config" "current" {}

resource "azurerm_resource_group" "keyvault" {
  name     = var.resource_group_name
  location = var.location

  tags = {
    environment = var.environment
  }
}

resource "azurerm_key_vault" "secrets_vault" {
  name                = var.name
  location            = "uksouth"
  resource_group_name = azurerm_resource_group.keyvault.name
  tenant_id           = data.azurerm_client_config.current.tenant_id
  soft_delete_enabled = true

  sku_name = "standard"

  network_acls {
    default_action = var.network_acls_default_action
    bypass         = "AzureServices"
    ip_rules       = var.access_CIDR
  }

  tags = {
    environment = var.environment
  }

  dynamic "access_policy" {
    for_each = var.authorised_person_ids
    content {
      tenant_id = data.azurerm_client_config.current.tenant_id
      object_id = access_policy.value

      certificate_permissions = [
        "Get",
        "List",
        "Update",
        "Create",
        "Import",
        "Delete",
        "Recover",
        "Backup",
        "Restore",
        "ManageContacts",
        "ManageIssuers",
        "GetIssuers",
        "ListIssuers",
        "SetIssuers",
        "DeleteIssuers",
      ]

      key_permissions = [
        "Get",
        "List",
        "Update",
        "Create",
        "Import",
        "Delete",
        "Recover",
        "Backup",
        "Restore",
      ]

      secret_permissions = [
        "Get",
        "List",
        "Set",
        "Delete",
        "Recover",
        "Backup",
        "Restore",
      ]

      storage_permissions = []
    }
  }
}
