variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
}

variable "vnet_name" {
  description = "Virtual Network name"
}

variable "vnet_resource_group" {
  description = "Virtual Network resource group name"
}

variable "lb_subnet_id" {
  description = "Load Balancer Subnet id"
}

variable "cluster_subnet_name" {
  description = "Cluster Subnet name"
}

variable "cluster_subnet_cidr" {
  description = "Cluster CIDR block"
}

variable "cluster_route_destination_cidr_blocks" {
  type        = list(string)
  description = "CIDR block destination for default route"
}

variable "cluster_route_next_hop" {
  description = "Next hop for default route"
}

variable "lb_route_table_id" {
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

variable "default_node_count" {
  description = "Default number of nodes in AKS cluster"
  default     = "2"
}
