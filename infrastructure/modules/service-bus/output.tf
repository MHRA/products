output "service_bus_default_primary_connection_string" {
  value = azurerm_servicebus_namespace.doc_index_updater_service_bus.default_primary_connection_string
}

output "service_bus_default_primary_key" {
  value = azurerm_servicebus_namespace.doc_index_updater_service_bus.default_secondary_key
}
