output "products-primary-access-key" {
  value = azurerm_storage_account.products.primary_access_key
}

output "products-static-web-url" {
  value = azurerm_storage_account.products.primary_web_endpoint
}

output "cpd-primary-access-key" {
  value = azurerm_storage_account.cpd.primary_access_key
}

output "cpd-static-web-url" {
  value = azurerm_storage_account.cpd.primary_web_endpoint
}

output "search_admin_key" {
  value = azurerm_search_service.search.primary_key
}

output "client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}
