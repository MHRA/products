variable "resource_group_name" {
  description = "Resource group name"
}
variable "location" {
  description = "Resource group location"
}
variable "environment" {
  description = "Environment name to use as a tag"
}
variable "name" {
  description = "Name of the keyvault"
}
variable "access_CIDR" {
  description = "CIDR range to whitelist for access to the keyvault"
}

variable "authorised_person_ids" {
  description = "IDs of objects (people etc) to associate access policies for"
}

variable "network_acls_default_action" {
  description = "Default action when an IP address isn't found in the ip rules. Options: 'Allow' or 'Deny'"
}
