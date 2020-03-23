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
