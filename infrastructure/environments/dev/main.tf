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
    storage_account_name = "developmenttfstate"
    container_name       = "tfstate"
    key                  = "dev.terraform.tfstate"
  }
}

locals {
  namespace        = "mhraproducts${var.ENVIRONMENT}"
  cpd_namespace    = "mhracpd${var.ENVIRONMENT}"
  pars_namespace   = "mhrapars${var.ENVIRONMENT}"
  service_bus_name = "doc-index-updater-${var.ENVIRONMENT}"
  logs_namespace   = "mhralogs${var.ENVIRONMENT}"
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION

  tags = {
    environment = var.ENVIRONMENT
  }
}

resource "azurerm_subnet_route_table_association" "load_balancer" {
  subnet_id      = azurerm_subnet.load_balancer.id
  route_table_id = azurerm_route_table.load_balancer.id
}

data "azurerm_resource_group" "keyvault" {
  name = var.KEYVAULT_RESOURCE_GROUP
}

# website
module "products" {
  source = "../../modules/products"

  environment                        = var.ENVIRONMENT
  location                           = var.REGION
  namespace                          = local.namespace
  pars_namespace                     = local.pars_namespace
  resource_group_name                = azurerm_resource_group.products.name
  app_registration_owners            = var.KEYVAULT_AUTHORISED_PERSON_IDS
  additional_allowed_pars_reply_urls = ["http://localhost:3000"]
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

resource "azurerm_route_table" "load_balancer" {
  name                = local.namespace
  location            = azurerm_resource_group.products.location
  resource_group_name = azurerm_resource_group.products.name

  tags = {
    environment = var.ENVIRONMENT
  }
}

resource "azurerm_virtual_network" "cluster" {
  name                = "aparz-spoke-dev-products"
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  address_space       = ["10.5.65.128/25"]
}

resource "azurerm_subnet" "load_balancer" {
  name                 = "adarz-spoke-products-dev-sn-01"
  address_prefixes     = ["10.5.65.128/26"]
  resource_group_name  = azurerm_virtual_network.cluster.resource_group_name
  virtual_network_name = azurerm_virtual_network.cluster.name
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
  cluster_subnet_name                   = "adarz-spoke-products-dev-sn-02"
  cluster_subnet_cidr                   = "10.5.65.192/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  lb_route_table_id                     = azurerm_route_table.load_balancer.id
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = false
  logs_storage_account_id               = module.logs.logs_resource_group_id
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = local.service_bus_name
  resource_group_name = azurerm_resource_group.products.name
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
