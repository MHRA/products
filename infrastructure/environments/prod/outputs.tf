output "api_cluster_client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "api_cluster_kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}

output "api_cluster_public_ip" {
  value = azurerm_public_ip.products_ip.ip_address
}

output "api_cluster_host" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.host
}

output "api_cluster_resource_group_name" {
  value = azurerm_kubernetes_cluster.cluster.resource_group_name
}
