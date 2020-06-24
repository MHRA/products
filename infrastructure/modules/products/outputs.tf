output "products_primary_access_key" {
  value = azurerm_storage_account.products.primary_access_key
}

output "products_static_web_url" {
  value = azurerm_storage_account.products.primary_web_endpoint
}

output "search_admin_key" {
  value = azurerm_search_service.search.primary_key
}

output "search_id" {
  value = azurerm_search_service.search.id
}

output "storage_access_key" {
  value = azurerm_storage_account.products.primary_access_key
}

output "search_service_name" {
  value = azurerm_search_service.search.name
}

output "storage_account_name" {
  value = azurerm_storage_account.products.name
}

output "storage_account_primary_web_host" {
  value = azurerm_storage_account.products.primary_web_host
}

output "products_hostname_cdn" {
  value = azurerm_cdn_endpoint.products.host_name
}
