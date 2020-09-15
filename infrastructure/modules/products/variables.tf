variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
}

variable "environment" {
  description = "Environment name to use as a tag"
}

variable "namespace" {
  description = "Namespace to use on cluster and storage"
}

variable "search_sku" {
  description = "Search Service SKU (e.g. basic/standard)"
  default     = "basic"
}

variable "cdn_region" {
  description = "Region where the CDN profile should be deployed"
  default     = "westeurope" # uksouth is not a valid option currently for cdn profiles
}

variable "search_replicas" {
  description = "Number of replicas of the search service to provision. More replicas gives higher reliability and ability to service more requests"
  default     = 1
}
