variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
}

variable "vnet_name" {
  description = "Virtual Network name"
}

variable "subnet_name" {
  description = "Subnet name"
}

variable "address_space" {
  description = "CIDR block for cluster subnet"
}

variable "route_table_name" {
  description = "Route Table name"
}

variable "route_table_resource_group_name" {
  description = "Resource Group that hosts the Route Table"
}

variable "environment" {
  description = "Environment name to use as a tag"
}

variable "client_id" {
  description = "Service Principal Client ID"
}

variable "client_secret" {
  description = "Service Principal Client Secret"
}



