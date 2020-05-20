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

variable "redis_start_ip" {
  description = "First IP in allowed IP range"
}

variable "redis_end_ip" {
  description = "Last IP in allowed IP range"
}
