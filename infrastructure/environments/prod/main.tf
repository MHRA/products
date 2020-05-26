provider "azurerm" {
  version = "=2.8.0"
  features {}
}

provider "random" {
  version = "~> 2.2"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    storage_account_name = "mhraprodtfstate"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }
}

locals {
  namespace        = "mhraproducts${var.ENVIRONMENT}"
  pars_namespace   = "mhrapars${var.ENVIRONMENT}"
  service_bus_name = "doc-index-updater-${var.ENVIRONMENT}"
}

data "azurerm_resource_group" "products" {
  name = var.RESOURCE_GROUP_PRODUCTS
}

resource "azurerm_resource_group" "keyvault" {
  name     = var.KEYVAULT_RESOURCE_GROUP
  location = var.REGION

  tags = {
    environment = var.ENVIRONMENT
  }
}

# website
module "products" {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  pars_namespace      = local.pars_namespace
  resource_group_name = data.azurerm_resource_group.products.name
  search_sku          = "standard"
}

data "azurerm_route_table" "load_balancer" {
  name                = "aparz-spoke-rt-products-internal-only"
  resource_group_name = "asazr-rg-1001"
}

data "azurerm_virtual_network" "cluster" {
  name                = "aparz-spoke-pd-products"
  resource_group_name = "adazr-rg-1001"
}

data "azurerm_subnet" "load_balancer" {
  name                 = "aparz-spoke-products-sn-01"
  resource_group_name  = data.azurerm_virtual_network.cluster.resource_group_name
  virtual_network_name = data.azurerm_virtual_network.cluster.name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                             = var.CLIENT_ID
  client_secret                         = var.CLIENT_SECRET
  environment                           = var.ENVIRONMENT
  location                              = var.REGION
  resource_group_name                   = data.azurerm_resource_group.products.name
  vnet_name                             = data.azurerm_virtual_network.cluster.name
  vnet_resource_group                   = data.azurerm_virtual_network.cluster.resource_group_name
  lb_subnet_id                          = data.azurerm_subnet.load_balancer.id
  cluster_subnet_name                   = "aparz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.66.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  lb_route_table_id                     = data.azurerm_route_table.load_balancer.id
  default_node_count                    = "3"
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = true
  diagnostic_setting_name               = "production-cluster-diagnostics"
}

data "azurerm_public_ip" "external" {
  name                = split("/", module.cluster.load_balancer_public_outbound_ip_id)[8]
  resource_group_name = split("/", module.cluster.load_balancer_public_outbound_ip_id)[4]
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = local.service_bus_name
  resource_group_name = data.azurerm_resource_group.products.name
  redis_use_firewall  = true
  redis_firewall_ip   = data.azurerm_public_ip.external.ip_address
}

# Key vault
module keyvault {
  source = "../../modules/keyvault"

  environment                 = var.ENVIRONMENT
  location                    = var.REGION
  name                        = var.KEYVAULT_NAME
  resource_group_name         = azurerm_resource_group.keyvault.name
  access_CIDR                 = var.KEYVAULT_ACCESS_CIDR_BLOCKS
  authorised_person_ids       = var.KEYVAULT_AUTHORISED_PERSON_IDS
  network_acls_default_action = "Deny"
}
