output "kube_config" {
  value = module.cluster.kube_config
}

output "public_ip" {
  value = module.cluster.public_ip
}
output "host" {
  value = module.cluster.host
}

output "client_certificate" {
  value = module.cluster.client_certificate
}

output "resource_group_name" {
  value = module.cluster.resource_group_name
}

output "products-hostname-cdn" {
  value = azurerm_cdn_endpoint.products.host_name
}

output "cpd-hostname-cdn" {
  value = azurerm_cdn_endpoint.cpd.host_name
}
