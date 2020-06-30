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
    container_name       = "tfstate2"
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

# Website
module products {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.cpd_namespace
  resource_group_name = module.products.products_storage_account_name
  cdn_name            = module.products.products_cdn_name
}

# Logs
module logs {
  source = "../../modules/logs"

  namespace           = local.logs_namespace
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = module.products.products_storage_account_name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                             = var.CLIENT_ID
  client_secret                         = var.CLIENT_SECRET
  environment                           = var.ENVIRONMENT
  location                              = var.REGION
  resource_group_name                   = module.products.products_storage_account_name
  vnet_name                             = "aparz-spoke-np-products"
  vnet_cidr                             = "10.5.65.0/24"
  lb_subnet_name                        = "adarz-spoke-products-sn-01"
  lb_subnet_cidr                        = "10.5.65.0/26"
  cluster_subnet_name                   = "adarz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.65.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  default_node_count                    = "1"
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = false
  logs_storage_account_id               = module.logs.logs_resource_group_id
}

# Service Bus
module doc_index_updater {
  source = "../../modules/doc-index-updater"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = local.service_bus_name
  resource_group_name = module.products.products_storage_account_name
  redis_use_firewall  = false
  redis_firewall_ip   = module.cluster.cluster_outbound_ip
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
  network_acls_default_action = "Allow"
}

# PARs
module pars {
  source = "../../modules/pars"

  resource_group_name                = module.products.products_storage_account_name
  location                           = var.REGION
  environment                        = var.ENVIRONMENT
  namespace                          = local.pars_namespace
  cdn_name                           = module.products.products_cdn_name
  app_registration_owners            = var.ADMIN_PERSON_IDS
  additional_allowed_pars_reply_urls = []
}
