variable "resource_group_name" {
  description = "Resource group name"
}
variable "location" {
  description = "Resource group location"
}
variable "environment" {
  description = "Environment name to use as a tag"
}
variable "dns_zone_name" {
  description = "DNS zone name"
}
variable "doc_index_updater_record_name" {
  description = "Name of the doc index updater DNS record"
}
variable "medicines_api_record_name" {
  description = "Name of the medicines API DNS record"
}
variable "products_record_name" {
  description = "Name of the medicines API DNS record"
}
variable "cluster_public_inbound_ip_id" {
  description = "ID of the resource of the public IP of the cluster"
}
variable "products_cdn_id" {
  description = "ID of the resource of the CDN for products"
}
