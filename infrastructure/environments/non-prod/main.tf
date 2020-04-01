provider "azurerm" {
  version = "=2.2.0"
  features {}
}

provider "random" {
  version = "~> 2.2"
}

provider "null" {
  version = "~> 2.1"
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
  service_bus_name = "doc-index-updater-${var.ENVIRONMENT}"
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
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
  resource_group_name = azurerm_resource_group.products.name
}

data "azurerm_route_table" "load_balancer" {
  name                = "adarz-spoke-rt-products-internal-only"
  resource_group_name = "asazr-rg-1001"
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  vnet_name           = "aparz-spoke-np-products"
  vnet_cidr           = "10.5.65.0/24"
  lb_subnet_name      = "adarz-spoke-products-sn-01"
  lb_subnet_cidr      = "10.5.65.0/26"
  cluster_subnet_name = "adarz-spoke-products-sn-02"
  cluster_subnet_cidr = "10.5.65.64/26"
  route_table_id      = data.azurerm_route_table.load_balancer.id
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
