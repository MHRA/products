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

resource "random_integer" "deployment" {
  min = 1000
  max = 9999
}

locals {
  namespace        = "mhraproducts${random_integer.deployment.result}"
  cpd_namespace    = "mhracpd${random_integer.deployment.result}"
  pars_namespace   = "mhrapars${random_integer.deployment.result}"
  service_bus_name = "doc-index-updater-${random_integer.deployment.result}"
  logs_namespace   = "mhralogs${var.ENVIRONMENT}"
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION

  tags = {
    environment = var.ENVIRONMENT
  }
}

resource "azurerm_resource_group" "keyvault" {
  name     = var.KEYVAULT_RESOURCE_GROUP
  location = var.REGION

  tags = {
    environment = var.ENVIRONMENT
  }
}

resource "azurerm_route_table" "load_balancer" {
  name                = "aparz-spoke-rt-products-internal-only"
  location            = azurerm_resource_group.products.location
  resource_group_name = azurerm_resource_group.products.name

  tags = {
    environment = var.ENVIRONMENT
  }
}

resource "azurerm_subnet_route_table_association" "load_balancer" {
  subnet_id      = azurerm_subnet.load_balancer.id
  route_table_id = azurerm_route_table.load_balancer.id
}

resource "azurerm_virtual_network" "cluster" {
  name                = "aparz-spoke-pd-products"
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  address_space       = ["10.5.67.0/24"]
}

resource "azurerm_subnet" "load_balancer" {
  name                 = "aparz-spoke-products-sn-01"
  address_prefixes     = ["10.5.67.0/26"]
  resource_group_name  = azurerm_virtual_network.cluster.resource_group_name
  virtual_network_name = azurerm_virtual_network.cluster.name
}

# Website
module "products" {
  source = "../../modules/products"

  environment                       = var.ENVIRONMENT
  location                          = var.REGION
  namespace                         = local.namespace
  pars_namespace                    = local.pars_namespace
  resource_group_name               = azurerm_resource_group.products.name
  search_sku                        = "standard"
  app_registration_owners           = var.KEYVAULT_AUTHORISED_PERSON_IDS
  addtional_allowed_pars_reply_urls = ["https://pars.mhra.gov.uk"]
}

# website
module "products_web" {
  source = "../../modules/products-web"

  namespace            = local.namespace
  environment          = var.ENVIRONMENT
  storage_account_name = module.products.storage_account_name
  resource_group_name  = azurerm_resource_group.products.name
  origin_host_name     = module.products.storage_account_primary_web_host
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.cpd_namespace
  resource_group_name = azurerm_resource_group.products.name
}

# Logs
module logs {
  source = "../../modules/logs"

  namespace           = local.logs_namespace
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                             = var.CLIENT_ID
  client_secret                         = var.CLIENT_SECRET
  environment                           = var.ENVIRONMENT
  location                              = var.REGION
  resource_group_name                   = azurerm_resource_group.products.name
  vnet_name                             = azurerm_virtual_network.cluster.name
  vnet_resource_group                   = azurerm_virtual_network.cluster.resource_group_name
  lb_subnet_id                          = azurerm_subnet.load_balancer.id
  cluster_subnet_name                   = "aparz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.67.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  lb_route_table_id                     = azurerm_route_table.load_balancer.id
  default_node_count                    = "3"
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = true
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
  name                        = local.namespace
  resource_group_name         = azurerm_resource_group.keyvault.name
  access_CIDR                 = var.KEYVAULT_ACCESS_CIDR_BLOCKS
  authorised_person_ids       = var.KEYVAULT_AUTHORISED_PERSON_IDS
  network_acls_default_action = "Deny"
}
