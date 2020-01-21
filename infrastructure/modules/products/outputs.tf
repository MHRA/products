output "products-primary-access-key" {
  value = azurerm_storage_account.products.primary_access_key
}

output "products-static-web-url" {
  value = azurerm_storage_account.products.primary_web_endpoint
}
