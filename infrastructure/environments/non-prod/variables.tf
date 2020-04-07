variable "REGION" {
  description = "Azure region"
  default     = "uksouth"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "adazr-rg-1001"
}

variable "CLIENT_ID" {
  description = "Service Principal Client ID"
}

variable "CLIENT_SECRET" {
  description = "Service Principal Client Secret"
}

variable "ENVIRONMENT" {
  description = "Environment name"
  default     = "non-prod"
}

variable "CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS" {
  type        = list(string)
  description = "CIDR block destination for default route"
}

variable "CLUSTER_ROUTE_NEXT_HOP" {
  description = "Next hop for default route"
}
