output "queues_default_primary_connection_string" {
  value = tomap({
    for k, v in azurerm_servicebus_queue_authorization_rule.service_bus_queue_auth_rule :
    k => v.primary_connection_string
  })
}

output "queues_default_primary_key" {
  value = tomap({
    for k, v in azurerm_servicebus_queue_authorization_rule.service_bus_queue_auth_rule :
    k => v.primary_key
  })
}
