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
  description = "Name for redis resource"
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
