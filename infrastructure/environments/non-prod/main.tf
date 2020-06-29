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
    storage_account_name = "mhranonprodtfstate"
    container_name       = "tfstate"
    key                  = "non-prod.terraform.tfstate"
  }
}

locals {
  namespace        = "mhraproductsnonprod"
  cpd_namespace    = "mhracpdnonprod"
  pars_namespace   = "mhraparsnonprod"
  service_bus_name = "doc-index-updater-${var.ENVIRONMENT}"
  logs_namespace   = "mhralogsnonprod"
}

# resource "azurerm_resource_group" "products" {
#   name     = var.RESOURCE_GROUP_PRODUCTS
#   location = var.REGION

#   tags = {
#     environment = var.ENVIRONMENT
#   }
# }

# data "azurerm_resource_group" "keyvault" {
#   name = var.KEYVAULT_RESOURCE_GROUP
# }

# resource "azurerm_subnet_route_table_association" "load_balancer" {
#   subnet_id      = azurerm_subnet.load_balancer.id
#   route_table_id = data.azurerm_route_table.load_balancer.id
# }

# website
module "products" {
  source = "../../modules/products"

  environment                        = var.ENVIRONMENT
  location                           = var.REGION
  namespace                          = local.namespace
  pars_namespace                     = local.pars_namespace
  resource_group_name                = var.RESOURCE_GROUP_PRODUCTS
  app_registration_owners            = var.ADMIN_PERSON_IDS
  additional_allowed_pars_reply_urls = []
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.cpd_namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}
# data "azurerm_route_table" "load_balancer" {
#   name                = "adarz-spoke-rt-products-internal-only"
#   resource_group_name = "asazr-rg-1001"
# }

# data "azurerm_virtual_network" "cluster" {
#   name                = "aparz-spoke-np-products"
#   resource_group_name = "adazr-rg-1001"
# }

# resource "azurerm_subnet" "load_balancer" {
#   name                 = "adarz-spoke-products-sn-01"
#   address_prefixes     = ["10.5.65.0/26"]
#   resource_group_name  = data.azurerm_virtual_network.cluster.resource_group_name
#   virtual_network_name = data.azurerm_virtual_network.cluster.name
# }

# Logs
module logs {
  source = "../../modules/logs"

  namespace           = local.logs_namespace
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                             = var.CLIENT_ID
  client_secret                         = var.CLIENT_SECRET
  environment                           = var.ENVIRONMENT
  location                              = var.REGION
  resource_group_name                   = var.RESOURCE_GROUP_PRODUCTS
  vnet_name                             = data.azurerm_virtual_network.cluster.name
  vnet_resource_group                   = data.azurerm_virtual_network.cluster.resource_group_name
  lb_subnet_id                          = azurerm_subnet.load_balancer.id
  cluster_subnet_name                   = "adarz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.65.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  lb_route_table_id                     = data.azurerm_route_table.load_balancer.id
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = false
  logs_storage_account_id               = module.logs.logs_resource_group_id
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
  resource_group_name = azurerm_resource_group.products.name
  redis_use_firewall  = true
  redis_firewall_ip   = data.azurerm_public_ip.external.ip_address
}

# Key vault
module keyvault {
  source = "../../modules/keyvault"

  environment                 = var.ENVIRONMENT
  location                    = var.REGION
  name                        = var.KEYVAULT_NAME
  resource_group_name         = data.azurerm_resource_group.keyvault.name
  access_CIDR                 = var.KEYVAULT_ACCESS_CIDR_BLOCKS
  authorised_person_ids       = var.KEYVAULT_AUTHORISED_PERSON_IDS
  network_acls_default_action = "Allow"
}
