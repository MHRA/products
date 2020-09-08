output "nibsc_primary_access_key" {
  value = azurerm_storage_account.nibsc.primary_access_key
}

output "nibsc_static_web_url" {
  value = azurerm_storage_account.nibsc.primary_web_endpoint
}

output "nibsc_hostname_cdn" {
  value = azurerm_cdn_endpoint.nibsc.host_name
}
