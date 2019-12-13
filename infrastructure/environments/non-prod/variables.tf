variable "STORAGE_ACCOUNT_NAME" {
  description = "Name of the Azure Storage Account used for Terraform state"
}

variable "CONTAINER_NAME" {
  description = "Name of the Azure Blob Storage Container used for Terraform state"
}

variable "ACCESS_KEY" {
  description = "Access Key for the Azure Storage Account used for Terraform state"
}

variable "REGION" {
  description = "Azure region"
  default     = "uksouth"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "products"
}
