resource "azurerm_monitor_scheduled_query_rules_alert" "medicines_api_errors_alert" {
  name                = "Medicine API Errors (${var.environment})"
  location            = var.location
  resource_group_name = var.resource_group_name

  action {
    action_group           = []
    email_subject          = "Application ERRORS"
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
  | project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID
  | render table
  | extend message_ = tostring(parse_json(tostring(parse_json(LogEntry).fields)).message)
  | where parse_json(LogEntry).level == "WARN" or parse_json(LogEntry).level == "ERROR"
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SERVICE_BUS_MESSAGE_LOCK_TIMEOUT to \"10\" (environment variable not found)"
  | extend correlation_id_ = tostring(parse_json(tostring(parse_json(LogEntry).span)).correlation_id)
  | extend level = tostring(parse_json(tostring(parse_json(LogEntry).level)))
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SECONDS_TO_WAIT to \"5\" (environment variable not found)"
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SECONDS_TO_WAIT to \"5\" (environment variable not found)"
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
    action_group           = []
    email_subject          = "Application ERRORS"
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
  | project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID
  | render table
  | extend message_ = tostring(parse_json(tostring(parse_json(LogEntry).fields)).message)
  | where parse_json(LogEntry).level == "WARN" or parse_json(LogEntry).level == "ERROR"
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SERVICE_BUS_MESSAGE_LOCK_TIMEOUT to \"10\" (environment variable not found)"
  | extend correlation_id_ = tostring(parse_json(tostring(parse_json(LogEntry).span)).correlation_id)
  | extend level = tostring(parse_json(tostring(parse_json(LogEntry).level)))
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SECONDS_TO_WAIT to \"5\" (environment variable not found)"
  | where parse_json(tostring(parse_json(LogEntry).fields)).message <> "defaulting SECONDS_TO_WAIT to \"5\" (environment variable not found)"
  QUERY
  severity       = 1
  frequency      = 5
  time_window    = 10
  trigger {
    operator  = "GreaterThan"
    threshold = 10
  }
}
