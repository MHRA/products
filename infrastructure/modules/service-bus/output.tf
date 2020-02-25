output "queues_default_primary_connection_string" {
  value = ["${azurerm_servicebus_queue_authorization_rule.doc_index_updater_service_bus_queue_auth_rule.*.primary_connection_string}"]
}

output "queues_default_primary_key" {
  value = ["${azurerm_servicebus_queue_authorization_rule.doc_index_updater_service_bus_queue_auth_rule.*.primary_key}"]
}
