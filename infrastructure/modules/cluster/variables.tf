variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
}

variable "vnet_name" {
  description = "Virtual Network name"
}

variable "vnet_cidr" {
  description = "CIDR block for cluster vnet"
}

variable "lb_subnet_name" {
  description = "Load Balancer Subnet name"
}

variable "lb_subnet_cidr" {
  description = "Load Balancer CIDR block"
}

variable "cluster_subnet_name" {
  description = "Cluster Subnet name"
}

variable "cluster_subnet_cidr" {
  description = "Cluster CIDR block"
}

variable "route_table_id" {
  description = "Route Table ID"
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
