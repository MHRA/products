output "kube_config" {
  value = module.cluster.kube_config
}

output "public_ip" {
  value = module.cluster.public_ip
}
output "host" {
  value = module.cluster.host
}
