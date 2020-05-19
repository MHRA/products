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

variable "pars_reply_urls" {
  description = "The reply urls configured in the azure app registration"
}
