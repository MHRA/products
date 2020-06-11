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

variable "pars_namespace" {
  description = "Namespace to use on cluster and storage for pars website"
}

variable "search_sku" {
  description = "Search Service SKU (e.g. basic/standard)"
  default     = "basic"
}

variable "addtional_allowed_pars_reply_urls" {
  description = "The CDN url and the primary static website URLs will be added by default, these are the addtional urls"
}

variable "app_registration_owners" {
  description = "Users who can update the app registration settings"
}

variable "include_pars_app" {
  description = "Include PARs app registration in the managed resources for this environment"
  default     = true
}
