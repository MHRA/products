output "logs_primary_access_key" {
  value = azurerm_storage_account.logs.primary_access_key
}

output "logs_resource_group_id" {
  value = azurerm_storage_account.logs.id
}
