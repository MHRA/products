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
  pars_namespace   = "mhraparsnonprod"
  service_bus_name = "doc-index-updater-${var.ENVIRONMENT}"
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
  route_table_id = data.azurerm_route_table.load_balancer.id
}


# website
module "products" {
  source = "../../modules/products"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = local.namespace
  pars_namespace      = local.pars_namespace
  resource_group_name = azurerm_resource_group.products.name
  pars_reply_urls     = var.PARS_REPLY_URLS
}

# website
module "products_web" {
  source = "../../modules/products-web"

  environment          = var.ENVIRONMENT
  storage_account_name = module.products.storage_account_name
  resource_group_name  = azurerm_resource_group.products.name
  origin_host_name     = module.products.storage_account_primary_web_host
}

data "azurerm_route_table" "load_balancer" {
  name                = "adarz-spoke-rt-products-internal-only"
  resource_group_name = "asazr-rg-1001"
}

data "azurerm_virtual_network" "cluster" {
  name                = "aparz-spoke-np-products"
  resource_group_name = "adazr-rg-1001"
}

resource "azurerm_subnet" "load_balancer" {
  name                 = "adarz-spoke-products-sn-01"
  address_prefixes     = ["10.5.65.0/26"]
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
  resource_group_name                   = azurerm_resource_group.products.name
  vnet_name                             = data.azurerm_virtual_network.cluster.name
  vnet_resource_group                   = data.azurerm_virtual_network.cluster.resource_group_name
  lb_subnet_id                          = azurerm_subnet.load_balancer.id
  cluster_subnet_name                   = "adarz-spoke-products-sn-02"
  cluster_subnet_cidr                   = "10.5.65.64/26"
  cluster_route_destination_cidr_blocks = var.CLUSTER_ROUTE_DESTINATION_CIDR_BLOCKS
  cluster_route_next_hop                = var.CLUSTER_ROUTE_NEXT_HOP
  lb_route_table_id                     = data.azurerm_route_table.load_balancer.id
  support_email_addresses               = var.SUPPORT_EMAIL_ADDRESSES
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = "mhracpdnonprod"
  resource_group_name = azurerm_resource_group.products.name
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = local.service_bus_name
  resource_group_name = azurerm_resource_group.products.name
}
