
output "products_hostname_cdn" {
  value = azurerm_cdn_endpoint.products.host_name
}
output "products_cdn_id" {
  value = azurerm_cdn_endpoint.products.id
}
