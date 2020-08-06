output "client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}

output "public_ip" {
  value = azurerm_public_ip.products_ip.ip_address
}

output "public_ip_id" {
  value = azurerm_public_ip.products_ip.id
}

output "host" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.host
}

output "resource_group_name" {
  value = azurerm_kubernetes_cluster.cluster.resource_group_name
}

output "cluster_outbound_ip" {
  value = data.azurerm_public_ip.cluster_outbound.ip_address
}
