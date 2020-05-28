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

variable "add_local_pars_reply_url" {
  description = "Whether to add http://localhost:3000 to allowable redirect urls"
  default     = false
}

variable "app_registration_owners" {
  description = "Users who can update the app registration settings"
}

variable "pars_app_name" {
  description = "Name of the PARS app"
}
