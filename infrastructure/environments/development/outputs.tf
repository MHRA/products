output "kube_config" {
  value = module.cluster.kube_config
}
output "container_registry_id" {
  value = module.cluster.container_registry_id
}
output "container_registry_login_server" {
  value = module.cluster.container_registry_login_server
}

output "public_ip" {
  value = module.cluster.public_ip
}
output "host" {
  value = module.cluster.host
}
