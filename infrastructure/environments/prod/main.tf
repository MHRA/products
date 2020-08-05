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
    storage_account_name = "tfstate10630"
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
  logs_namespace   = "mhralogs${random_integer.deployment.result}"
}

# Website
module products {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  search_sku          = "standard"
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.cpd_namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  cdn_name            = module.products.products_cdn_name
}

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
  vnet_name                             = "aparz-spoke-pd-products"
  vnet_cidr                             = "10.5.67.0/24"
  lb_subnet_name                        = "aparz-spoke-products-sn-01"
  lb_subnet_cidr                        = "10.5.67.0/26"
  lb_route_table_name                   = "aparz-spoke-rt-products-internal-only"
  cluster_subnet_name                   = "aparz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.67.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  default_node_count                    = "3"
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = true
  logs_storage_account_id               = module.logs.logs_resource_group_id
}

# Service Bus
module doc_index_updater {
  source = "../../modules/doc-index-updater"

  environment             = var.ENVIRONMENT
  location                = var.REGION
  name                    = local.service_bus_name
  resource_group_name     = var.RESOURCE_GROUP_PRODUCTS
  redis_use_firewall      = true
  redis_firewall_ip       = module.cluster.cluster_outbound_ip
  logs_storage_account_id = module.logs.logs_resource_group_id
}

# Key vault
module keyvault {
  source = "../../modules/keyvault"

  environment                 = var.ENVIRONMENT
  location                    = var.REGION
  name                        = local.namespace
  resource_group_name         = var.KEYVAULT_RESOURCE_GROUP
  access_CIDR                 = var.KEYVAULT_ACCESS_CIDR_BLOCKS
  authorised_person_ids       = var.ADMIN_PERSON_IDS
  network_acls_default_action = "Deny"
}

# PARs
module pars {
  source = "../../modules/pars"

  resource_group_name                = var.RESOURCE_GROUP_PRODUCTS
  location                           = var.REGION
  environment                        = var.ENVIRONMENT
  namespace                          = local.pars_namespace
  cdn_name                           = module.products.products_cdn_name
  app_registration_owners            = var.ADMIN_PERSON_IDS
  additional_allowed_pars_reply_urls = ["https://pars.mhra.gov.uk"]
  include_pars_app                   = false
}
