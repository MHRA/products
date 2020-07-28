resource "azurerm_resource_group" "dns" {
  name     = var.resource_group_name
  location = var.location

  tags = {
    environment = var.environment
  }
}

resource "azurerm_dns_zone" "dns_public" {
  name                = var.dns_zone_name
  resource_group_name = azurerm_resource_group.dns.name

  tags = {
    environment = var.environment
  }
}

resource "azurerm_dns_a_record" "doc-index-updater" {
  name                = var.doc_index_updater_record_name
  zone_name           = azurerm_dns_zone.dns_public.name
  resource_group_name = azurerm_resource_group.dns.name
  ttl                 = 3600
  target_resource_id  = var.cluster_public_ip_id
}
