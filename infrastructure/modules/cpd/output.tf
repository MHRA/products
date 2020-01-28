output "cpd-primary-access-key" {
  value = azurerm_storage_account.cpd.primary_access_key
}

output "cpd-static-web-url" {
  value = azurerm_storage_account.cpd.primary_web_endpoint
}
