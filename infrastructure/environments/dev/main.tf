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
    storage_account_name = "developmenttfstate"
    container_name       = "tfstate"
    key                  = "dev.terraform.tfstate"
  }
}

locals {
  namespace = "mhraproductsdevelopment"
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

resource "azurerm_route_table" "load_balancer" {
  name                = local.namespace
  location            = azurerm_resource_group.products.location
  resource_group_name = azurerm_resource_group.products.name

  tags = {
    environment = var.ENVIRONMENT
  }
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id           = var.CLIENT_ID
  client_secret       = var.CLIENT_SECRET
  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.products.name
  vnet_name           = "aparz-spoke-dev-products"
  vnet_cidr           = "10.6.65.0/24"
  lb_subnet_name      = "adarz-spoke-products-dev-sn-01"
  lb_subnet_cidr      = "10.6.65.0/26"
  cluster_subnet_name = "adarz-spoke-products-dev-sn-02"
  cluster_subnet_cidr = "10.6.65.64/26"
  route_table_id      = azurerm_route_table.load_balancer.id
}

# CPD
# module cpd {
#   source = "../../modules/cpd"

#   environment         = var.ENVIRONMENT
#   location            = var.REGION
#   namespace           = local.namespace
#   resource_group_name = azurerm_resource_group.products.name
# }

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = "doc-index-updater-${var.ENVIRONMENT}"
  resource_group_name = azurerm_resource_group.products.name
}
