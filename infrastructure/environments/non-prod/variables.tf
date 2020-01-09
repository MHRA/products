variable "REGION" {
  description = "Azure region"
  default     = "uksouth"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "products"
}

variable "CLIENT_ID" {
  description = "Service Principal Client ID"
}

variable "CLIENT_SECRET" {
  description = "Service Principal Client Secret"
}

variable "ENVIROMENT" {
  description = "Enviroment name"
  default     = "non-prod"
}
