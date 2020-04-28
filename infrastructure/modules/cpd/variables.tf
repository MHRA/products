variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
}

variable "cdn_location" {
  description = "Region to create the CDN in"
}

variable "environment" {
  description = "Environment name to use as a tag"
}

variable "namespace" {
  description = "Namespace to use on cluster and storage"
}
