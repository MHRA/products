variable "storage_account_name" {
  description = "Storage account name where the $web storage container should be added"
}

variable "resource_group_name" {
  description = "Resource group for the CDN profile"
}

variable "origin_host_name" {
  description = "Origin host name for the CDN endpoint"
}

variable "cdn_region" {
  description = "Region where the CDN profile should be deployed"
  default     = "westeurope" # uksouth is not a valid option currently for cdn profiles
}

variable "environment" {
  description = "The name of the environment we are deploying to"
}
