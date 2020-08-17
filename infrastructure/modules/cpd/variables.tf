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
  description = "Namespace to use on resources"
}
variable "cdn_name" {
  description = "Name of the CDN resource to associate the CDN endpoint with"
}
variable "cdn_region" {
  description = "Region where the CDN profile should be deployed"
  default     = "westeurope" # uksouth is not a valid option currently for cdn profiles
}
