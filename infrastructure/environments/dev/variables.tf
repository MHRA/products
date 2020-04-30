variable "REGION" {
  description = "Azure region"
  default     = "westeurope"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "mhra-products-development"
}

variable "CLIENT_ID" {
  description = "Service Principal Client ID"
}

variable "CLIENT_SECRET" {
  description = "Service Principal Client Secret"
}

variable "ENVIRONMENT" {
  description = "Environment name"
  default     = "dev"
}

variable "CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS" {
  type        = list(string)
  description = "CIDR block destination for default route"
}

variable "CLUSTER_ROUTE_NEXT_HOP" {
  description = "Next hop for default route"
}

variable "SUPPORT_EMAIL_ADDRESSES" {
  type        = list(string)
  description = "A list of email addresses for first line support alerts to be sent to."
}
