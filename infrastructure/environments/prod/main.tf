provider "azurerm" {
  version = "~> 1.38.0"
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
    storage_account_name = "mhraprodtfstate"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }
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
  namespace           = "mhraproductsprod"
  resource_group_name = azurerm_resource_group.products.name
}

# AKS
module cluster {
  source = "../../modules/cluster"

  client_id                       = var.CLIENT_ID
  client_secret                   = var.CLIENT_SECRET
  environment                     = var.ENVIRONMENT
  location                        = var.REGION
  resource_group_name             = azurerm_resource_group.products.name
  vnet_name                       = "aparz-spoke-pd-products"
  vnet_cidr                       = "10.5.66.0/24"
  lb_subnet_name                  = "aparz-spoke-products-sn-01"
  lb_subnet_cidr                  = "10.5.66.0/26"
  cluster_subnet_name             = "aparz-spoke-products-sn-02"
  cluster_subnet_cidr             = "10.5.66.64/26"
  route_table_name                = "aparz-spoke-rt-products-internal-only"
  route_table_resource_group_name = "asazr-rg-1001"
}

# CPD
module cpd {
  source = "../../modules/cpd"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  namespace           = "mhracpdprod"
  resource_group_name = azurerm_resource_group.products.name
}

# Service Bus
module service_bus {
  source = "../../modules/service-bus"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  name                = "doc-index-updater-${var.ENVIRONMENT}"
  resource_group_name = azurerm_resource_group.products.name
}
