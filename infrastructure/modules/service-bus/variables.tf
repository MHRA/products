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
  description = "name for the service bus namespace"
}

variable "redis_use_firewall" {
  type        = bool
  description = "Whether to apply a firewall rule for Redis"
  default     = false
}

variable "redis_firewall_ip" {
  description = "IP allowed to access Redis Cache"
  default     = ""
}

variable "logs_storage_account_id" {
  description = "ID of the storage account to send service bus logs to"
}

variable "diagnostic_log_types" {
  description = "Set of log types to create configuration for"
  type        = list(string)
  default     = ["OperationalLogs"]
}
