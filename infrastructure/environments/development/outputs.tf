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
# Service Bus
output "service_bus_default_primary_connection_string" {
  value = module.service_bus.service_bus_default_primary_connection_string
}

output "service_bus_default_primary_key" {
  value = module.service_bus.service_bus_default_primary_key
}
