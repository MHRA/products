output "dip_cluster_client_certificate" {
  value = azurerm_kubernetes_cluster.doc_index_updater_cluster.kube_config.0.client_certificate
}

output "dip_cluster_kube_config" {
  value = azurerm_kubernetes_cluster.doc_index_updater_cluster.kube_config_raw
}

output "dip_cluster_public_ip" {
  value = azurerm_public_ip.doc_index_updater_ip.ip_address
}

output "dip_cluster_host" {
  value = azurerm_kubernetes_cluster.doc_index_updater_cluster.kube_config.0.host
}

output "dip_cluster_resource_group_name" {
  value = azurerm_kubernetes_cluster.doc_index_updater_cluster.resource_group_name
}
