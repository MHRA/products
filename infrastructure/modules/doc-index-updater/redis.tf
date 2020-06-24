resource "azurerm_redis_cache" "doc_index_updater_redis" {
  name = var.name

  capacity            = 0
  enable_non_ssl_port = false
  family              = "C"
  location            = var.location
  minimum_tls_version = "1.2"
  resource_group_name = var.resource_group_name
  sku_name            = "Standard"
}

resource "azurerm_redis_firewall_rule" "cluster" {
  count               = var.redis_use_firewall ? 1 : 0
  name                = "cluster_ip_range"
  redis_cache_name    = azurerm_redis_cache.doc_index_updater_redis.name
  resource_group_name = var.resource_group_name
  start_ip            = var.redis_firewall_ip
  end_ip              = var.redis_firewall_ip
}
