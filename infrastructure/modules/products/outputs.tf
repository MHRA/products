output "products_primary_access_key" {
  value = azurerm_storage_account.products.primary_access_key
}

output "products_static_web_url" {
  value = azurerm_storage_account.products.primary_web_endpoint
}

output "search_admin_key" {
  value = azurerm_search_service.search.primary_key
}
