output "client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}

output "container_registry_id" {
  value = azurerm_container_registry.container_registry.id
}

output "container_registry_login_server" {
  value = azurerm_container_registry.container_registry.login_server
}


output "public_ip" {
  value = azurerm_public_ip.products_ip.ip_address
}


output "host" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.host
}
