resource "azurerm_monitor_action_group" "support" {
  name                = "CriticalAlertsAction"
  resource_group_name = var.resource_group_name
  short_name          = "support"

  dynamic "email_receiver" {
    for_each = var.support_email_addresses
    content {
      name                    = email_receiver.value
      email_address           = email_receiver.value
      use_common_alert_schema = true
    }
  }
}

resource "azurerm_monitor_scheduled_query_rules_alert" "medicines_api_errors_alert" {
  name                = "Medicine API Errors (${var.environment})"
  location            = var.location
  resource_group_name = var.resource_group_name

  action {
    action_group           = [azurerm_monitor_action_group.support.id]
    email_subject          = "Medicine API Errors (${var.environment})"
    custom_webhook_payload = "{}"
  }
  data_source_id = azurerm_log_analytics_workspace.cluster.id
  description    = "Alert when total results cross threshold"
  enabled        = true
  query          = <<-QUERY
  let clusterId = '${azurerm_kubernetes_cluster.cluster.id}';
  let ContainerIdList = KubePodInventory
  | where ContainerName contains 'medicines-api'
  | where ClusterId =~ clusterId
  | distinct ContainerID;
  ContainerLog
  | where ContainerID in (ContainerIdList)
  | project parse_json(LogEntry), TimeGenerated, ContainerID
  | render table
  | extend correlation_id = tostring(LogEntry.span.correlation_id)
  | extend message = tostring(LogEntry.span.message)
  | extend level = tostring(LogEntry.level)
  | where level == "ERROR"

  QUERY
  severity       = 1
  frequency      = 5
  time_window    = 10
  trigger {
    operator  = "GreaterThan"
    threshold = 10
  }
}

resource "azurerm_monitor_scheduled_query_rules_alert" "doc_index_updater_errors_alert" {
  name                = "Doc Index Updater Errors (${var.environment})"
  location            = var.location
  resource_group_name = var.resource_group_name

  action {
    action_group           = [azurerm_monitor_action_group.support.id]
    email_subject          = "Doc Index Updater Errors (${var.environment})"
    custom_webhook_payload = "{}"
  }
  data_source_id = azurerm_log_analytics_workspace.cluster.id
  description    = "Alert when total results cross threshold"
  enabled        = true
  query          = <<-QUERY
  let clusterId = '${azurerm_kubernetes_cluster.cluster.id}';
  let ContainerIdList = KubePodInventory
  | where ContainerName contains 'doc-index-updater'
  | where ClusterId =~ clusterId
  | distinct ContainerID;
  ContainerLog
  | where ContainerID in (ContainerIdList)
  | project parse_json(LogEntry), TimeGenerated, ContainerID
  | render table
  | extend correlation_id = tostring(LogEntry.span.correlation_id)
  | extend message = tostring(LogEntry.span.message)
  | extend level = tostring(LogEntry.level)
  | where level == "ERROR"
  QUERY
  severity       = 1
  frequency      = 5
  time_window    = 10
  trigger {
    operator  = "GreaterThan"
    threshold = 10
  }
}
