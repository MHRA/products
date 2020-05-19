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

output "search_service_name" {
  # TODO: fragile
  value = local.namespace
}

output "search_admin_key" {
  value = module.products.search_admin_key
}

output "service_bus_queue_keys" {
  value = module.service_bus.queues_default_primary_key
}

output "redis_access_key" {
  value = module.service_bus.redis_access_key
}

output "service_bus_name" {
  value = local.service_bus_name
}

output "storage_account_name" {
  # TODO: fragile
  value = local.namespace
}

output "storage_master_key" {
  value = module.products.storage_access_key
}

output "sas_url_query_string" {
  value = module.products.sas_url_query_string
}
