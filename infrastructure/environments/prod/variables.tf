variable "REGION" {
  description = "Azure region"
  default     = "uksouth"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "apazr-rg-1001"
}

variable "CLIENT_ID" {
  description = "Service Principal Client ID"
}

variable "CLIENT_SECRET" {
  description = "Service Principal Client Secret"
}

variable "ENVIRONMENT" {
  description = "Environment name"
  default     = "production"
}

variable "CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS" {
  type        = list(string)
  description = "CIDR block destination for default route"
  default     = []
}

variable "CLUSTER_ROUTE_NEXT_HOP" {
  description = "Next hop for default route"
}

variable "SUPPORT_EMAIL_ADDRESSES" {
  type        = list(string)
  description = "A list of email addresses for first line support alerts to be sent to."
  default     = []
}

variable "KEYVAULT_ACCESS_CIDR_BLOCKS" {
  type        = list(string)
  description = "CIDR blocks representing whitelisted IPs to access keyvault"
  default     = []
}

variable "ADMIN_PERSON_IDS" {
  type        = list(string)
  description = "IDs of objects (people etc) to associate access policies for"
  default     = []
}

variable "KEYVAULT_RESOURCE_GROUP" {
  description = "Name of resource group where keyvault is deployed"
  default     = "secrets"
}

variable "DNS_ZONE_NAME" {
  description = "Name of DNS zone"
  default     = "api.mhra.gov.uk"
}

variable "DNS_RESOURCE_GROUP_NAME" {
  description = "Name of resource group where DNS zone is deployed"
  default     = "dns"
}
