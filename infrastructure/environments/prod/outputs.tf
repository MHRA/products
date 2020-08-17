output "kube_config" {
  value = module.cluster.kube_config
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

output "search_service_name" {
  value = module.products.search_service_name
}

output "service_bus_queue_keys" {
  value = module.service_bus.queues_default_primary_key
}

output "redis_access_key" {
  value = module.redis.redis_access_key
}

output "service_bus_name" {
  value = local.doc_index_updater_namespace
}

output "storage_account_name" {
  value = module.products.storage_account_name
}

output "storage_master_key" {
  value = module.products.storage_access_key
}

output "logs_resource_group_id" {
  value = module.logs.logs_resource_group_id
}

output "logs_primary_access_key" {
  value = module.logs.logs_primary_access_key
}
