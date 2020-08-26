data "azurerm_subscription" "current" {}

resource "azurerm_dashboard" "doc-index-updater-dashboard" {
  name                = "doc-index-updater-dashboard-${var.environment}"
  resource_group_name = var.resource_group_name
  location            = var.location

  tags = {
    hidden-title = "Doc index updater ${var.environment}"
    Environment  = var.environment
  }

  dashboard_properties = <<DASH
{
  "lenses": {
    "0": {
      "order": 0,
      "parts": {
        "0": {
          "position": {
            "x": 0,
            "y": 0,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:cpu_usage_seconds_per_pod:mean\" \n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val) by tostring(parse_json(Tags).pod), bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "1133a37d-147d-402f-aa91-5bf38b90433d"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "CPU usage",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "1": {
          "position": {
            "x": 6,
            "y": 0,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:memory_usage_percent_per_pod:mean\"\n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val * 100) by tostring(parse_json(Tags).pod), bin(TimeGenerated, 1m)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "13d086d1-9020-4df2-9b10-d3bb2fe95949"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Memory usage (as % of limit)",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "2": {
          "position": {
            "x": 0,
            "y": 4,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:incoming_requests_per_second_per_pod:mean\" \n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val) by tostring(parse_json(Tags).pod_name), bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT4H"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "831dc9d4-07dd-4b3f-a624-2232212ba5b4"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Requests per second",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "3": {
          "position": {
            "x": 6,
            "y": 4,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:request_error_rate_per_pod:mean\" \n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val * 100) by tostring(parse_json(Tags).pod_name), bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "156e0229-154e-45cc-8c81-4e309ec207d2"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "HTTP request error rate (% of 5xx response codes)",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "4": {
          "position": {
            "x": 0,
            "y": 8,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:success_response_latency_milliseconds_per_pod:mean\" \n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val) by tostring(parse_json(Tags).pod_name), bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT4H"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "2d9c936c-048b-43f3-82de-59d096d4013e"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Successful request latency (ms)",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "5": {
          "position": {
            "x": 6,
            "y": 8,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:error_response_latency_milliseconds_per_pod:mean\" \n| where parse_json(Tags).namespace == \"doc-index-updater\"\n| summarize avg(Val) by tostring(parse_json(Tags).pod_name), bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "avg_Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "Tags_pod_name",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "14fbaca6-515b-42e1-9e14-7392ad21d3f2"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Error request latency (ms)",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "6": {
          "position": {
            "x": 0,
            "y": 12,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "let messageContains = \"Successfully added\"\n;let clusterId = 'providers/Microsoft.ContainerService/managedClusters/non-prod';\nlet ContainerIdList = KubePodInventory\n| where ContainerName contains 'doc-index-updater'\n| where ClusterId contains clusterId\n| distinct ContainerID;\nContainerLog\n| where ContainerID in (ContainerIdList)\n| project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID\n| order by TimeGenerated desc\n| render table\n| where parse_json(tostring(parse_json(LogEntry).fields)).message contains messageContains\n| summarize count() by bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "count_",
                      "type": "long"
                    }
                  ],
                  "splitBy": [],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "0a3d5244-4ea2-4eb5-bf6b-694d1848df0a"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Doc index updater successful uploads",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "7": {
          "position": {
            "x": 6,
            "y": 12,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "let messageContains = \"Successfully deleted\";\nlet ContainerIdList = KubePodInventory\n| where ContainerName contains 'doc-index-updater'\n| distinct ContainerID;\nContainerLog\n| where ContainerID in (ContainerIdList)\n| project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID\n| order by TimeGenerated desc\n| render table\n| where parse_json(tostring(parse_json(LogEntry).fields)).message contains messageContains\n| summarize count() by bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "count_",
                      "type": "long"
                    }
                  ],
                  "splitBy": [],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "108bc676-1974-4e64-a210-cab9fcb0535c"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Doc index updater successful deletes",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "8": {
          "position": {
            "x": 0,
            "y": 16,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "let ContainerIdList = KubePodInventory\n| where ContainerName contains 'doc-index-updater'\n| distinct ContainerID;\nContainerLog\n| where ContainerID in (ContainerIdList)\n| project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID\n| order by TimeGenerated desc\n| render table\n| extend message_ = tostring(parse_json(tostring(parse_json(LogEntry).fields)).message) \n| where parse_json(LogEntry).level == \"ERROR\"\n| summarize count() by bin(TimeGenerated, 15s)\n| render timechart\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "count_",
                      "type": "long"
                    }
                  ],
                  "splitBy": [],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "50e6e7b1-8139-4368-802f-45cb615553d2"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Doc index updater server errors",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "9": {
          "position": {
            "x": 6,
            "y": 16,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "${data.azurerm_subscription.current.subscription_id}",
                  "ResourceGroup": "${var.resource_group_name}",
                  "Name": "${azurerm_log_analytics_workspace.cluster.name}",
                  "ResourceId": "/subscriptions/${data.azurerm_subscription.current.subscription_id}/resourcegroups/${var.resource_group_name}/providers/microsoft.operationalinsights/workspaces/${azurerm_log_analytics_workspace.cluster.name}"
                }
              },
              {
                "name": "Query",
                "value": "let ContainerIdList = KubePodInventory\n| where ContainerName contains 'doc-index-updater'\n| distinct ContainerID;\nContainerLog\n| where ContainerID in (ContainerIdList)\n| project LogEntrySource, LogEntry, TimeGenerated, Computer, Image, Name, ContainerID\n| order by TimeGenerated desc\n| render table\n| where parse_json(LogEntry).level == \"ERROR\"\n| project TimeGenerated, tostring(parse_json(tostring(parse_json(LogEntry).fields)).message)\n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "count_",
                      "type": "long"
                    }
                  ],
                  "splitBy": [],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "44b942d7-0dd3-4979-b729-05f2d156001f"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "${azurerm_log_analytics_workspace.cluster.name}"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsGrid"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Doc index updater server errors list",
                "PartSubTitle": "${azurerm_log_analytics_workspace.cluster.name}"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        }
      }
    }
  },
  "metadata": {
    "model": {
      "timeRange": {
        "value": {
          "relative": {
            "duration": 24,
            "timeUnit": 1
          }
        },
        "type": "MsPortalFx.Composition.Configuration.ValueTypes.TimeRange"
      },
      "filterLocale": {
        "value": "en-us"
      },
      "filters": {
        "value": {
          "MsPortalFx_TimeRange": {
            "model": {
              "format": "utc",
              "granularity": "auto",
              "relative": "1h"
            },
            "displayCache": {
              "name": "UTC Time",
              "value": "Past hour"
            },
            "filteredPartIds": [
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f428f",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f4291",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f4293",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f4295",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f4297",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f4299",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f429b",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f429d",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f429f",
              "StartboardPart-AnalyticsPart-ffee39a5-76e8-4033-800a-344cb47f42a1"
            ]
          }
        }
      }
    }
  }
}
DASH
}
