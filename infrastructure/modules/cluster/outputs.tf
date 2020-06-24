output "client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}

output "host" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.host
}

output "resource_group_name" {
  value = azurerm_kubernetes_cluster.cluster.resource_group_name
}

output "cluster_outbound_ip" {
  value = split("/", tolist(azurerm_kubernetes_cluster.cluster.network_profile[0].load_balancer_profile[0].effective_outbound_ips)[0])[8]
}
