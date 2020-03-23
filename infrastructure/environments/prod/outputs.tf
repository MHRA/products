output "products_primary_access_key" {
  value = module.products.products_primary_access_key
}

output "products_static_web_url" {
  value = module.products.products_static_web_url
}

output "search_admin_key" {
  value = module.products.search_admin_key
}

output "cpd_primary_access_key" {
  value = module.cpd.cpd_primary_access_key
}

output "cpd_static_web_url" {
  value = module.cpd.cpd_static_web_url
}

output "products-hostname-cdn" {
  value = azurerm_cdn_endpoint.products.host_name
}

output "cpd-hostname-cdn" {
  value = azurerm_cdn_endpoint.cpd.host_name
}
