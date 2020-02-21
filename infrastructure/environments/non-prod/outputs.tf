# API Cluster

output "api_cluster_kube_config" {
  value = module.cluster.api_cluster_kube_config
}

output "api_cluster_public_ip" {
  value = module.cluster.api_cluster_public_ip
}
output "api_cluster_host" {
  value = module.cluster.api_cluster_host
}

output "api_cluster_client_certificate" {
  value = module.cluster.api_cluster_client_certificate
}

output "api_cluster_resource_group_name" {
  value = module.cluster.api_cluster_resource_group_name
}

# Document Index Updater cluster

output "dip_cluster_client_certificate" {
  value = module.doc_index_updater.dip_cluster_client_certificate
}

output "dip_cluster_kube_config" {
  value = module.doc_index_updater.dip_cluster_kube_config
}

output "dip_cluster_public_ip" {
  value = module.doc_index_updater.dip_cluster_public_ip
}

output "dip_cluster_host" {
  value = module.doc_index_updater.dip_cluster_host
}

output "dip_cluster_resource_group_name" {
  value = module.doc_index_updater.dip_cluster_resource_group_name
}
