variable "resource_group_name" {
  description = "Resource group name"
}

variable "location" {
  description = "Resource group location"
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

variable "support_email_addresses" {
  description = "A list of email addresses for first line support alerts to be sent to."
}

variable "log_cluster_diagnostics" {
  type        = bool
  description = "Whether diagnostics from the cluster should be logged to a storage container"
  default     = false
}

variable "diagnostic_log_types" {
  description = "Set of log types to create configuration for"
  type        = list(string)
  default = ["kube-apiserver",
    "kube-audit",
    "kube-controller-manager",
    "kube-scheduler",
  "cluster-autoscaler"]
}

variable "logs_storage_account_id" {
  description = "ID of the immutable storage account used for logs"
  default     = ""
}

variable "vnet_name" {
  description = "Name of the VNET that the cluster lives in"
  default     = ""
}

variable "vnet_cidr" {
  description = "CIDR IP range used for the cluster VNET"
  default     = ""
}

variable "lb_subnet_name" {
  description = "Name of the subnet that the internal load balancer lives in"
  default     = ""
}

variable "lb_subnet_cidr" {
  description = "CIDR IP range used for the subnet that the internal load balancer lives in"
  default     = ""
}

variable "lb_route_table_name" {
  description = "Name to use for the load balancer route table"
}
