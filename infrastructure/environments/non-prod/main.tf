terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "=2.52.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 2.2"
    }
  }
}

provider "azurerm" {
  features {}
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
  namespace                   = "mhraproductsnonprod"
  cpd_namespace               = "mhracpdnonprod"
  pars_namespace              = "mhraparsnonprod"
  doc_index_updater_namespace = "doc-index-updater-${var.ENVIRONMENT}"
  logs_namespace              = "mhralogsnonprod"
  nibsc_namespace             = "mhranibscnonprod"
}

# Website
module "products" {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  search_replicas     = 1
}

# CPD
module "cpd" {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.cpd_namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  cdn_name            = module.products.products_cdn_name
}

# Logs
module "logs" {
  source = "../../modules/logs"

  namespace           = local.logs_namespace
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
}

# AKS
module "cluster" {
  source = "../../modules/cluster"

  client_id                             = var.CLIENT_ID
  client_secret                         = var.CLIENT_SECRET
  environment                           = var.ENVIRONMENT
  location                              = var.REGION
  resource_group_name                   = var.RESOURCE_GROUP_PRODUCTS
  vnet_name                             = "aparz-spoke-np-products"
  vnet_cidr                             = "10.5.65.0/24"
  lb_subnet_name                        = "adarz-spoke-products-sn-01"
  lb_subnet_cidr                        = "10.5.65.0/26"
  lb_route_table_name                   = "adarz-spoke-rt-products-internal-only"
  cluster_subnet_name                   = "adarz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.65.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  default_node_count                    = 3
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
  log_cluster_diagnostics               = false
  logs_storage_account_id               = module.logs.logs_resource_group_id
}

# Service Bus
module "service_bus" {
  source = "../../modules/service-bus"

  environment             = var.ENVIRONMENT
  location                = var.REGION
  name                    = local.doc_index_updater_namespace
  resource_group_name     = var.RESOURCE_GROUP_PRODUCTS
  logs_storage_account_id = module.logs.logs_resource_group_id
}

# Redis
module "redis" {
  source = "../../modules/redis"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = local.doc_index_updater_namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  redis_use_firewall  = false
  redis_firewall_ip   = module.cluster.cluster_outbound_ip
}

# Key vault
module "keyvault" {
  source = "../../modules/keyvault"

  environment                 = var.ENVIRONMENT
  location                    = var.REGION
  name                        = "mhra-non-prod-02"
  resource_group_name         = var.KEYVAULT_RESOURCE_GROUP
  access_CIDR                 = var.KEYVAULT_ACCESS_CIDR_BLOCKS
  authorised_person_ids       = var.ADMIN_PERSON_IDS
  network_acls_default_action = "Allow"
}

# PARs
module "pars" {
  source = "../../modules/pars"

  resource_group_name                = var.RESOURCE_GROUP_PRODUCTS
  location                           = var.REGION
  environment                        = var.ENVIRONMENT
  namespace                          = local.pars_namespace
  cdn_name                           = module.products.products_cdn_name
  app_registration_owners            = var.ADMIN_PERSON_IDS
  additional_allowed_pars_reply_urls = []
}

# DNS
module "dns" {
  source = "../../modules/dns"

  environment                   = var.ENVIRONMENT
  location                      = var.REGION
  dns_zone_name                 = var.DNS_ZONE_NAME
  resource_group_name           = var.DNS_RESOURCE_GROUP_NAME
  cluster_public_inbound_ip_id  = module.cluster.cluster_public_inbound_ip_id
  doc_index_updater_record_name = "doc-index-updater"
  medicines_api_record_name     = "medicines-api"
  products_record_name          = "products"
  products_cdn_id               = module.products.products_cdn_id
}

# NIBSC
module "nibsc" {
  source = "../../modules/nibsc"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.nibsc_namespace
  resource_group_name = var.RESOURCE_GROUP_PRODUCTS
  cdn_name            = module.products.products_cdn_name
}
