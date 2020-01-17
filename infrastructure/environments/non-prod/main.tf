provider "azurerm" {
  version = "~> 1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    container_name       = "tfstate"
    storage_account_name = "tfstate4338"
    key                  = "non-prod.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "products" {
  name     = var.RESOURCE_GROUP_PRODUCTS
  location = var.REGION

  tags = {
    environment = var.ENVIROMENT
  }
}

resource "azurerm_storage_account" "products" {
  name                     = "mhraproductsnonprod"
  resource_group_name      = azurerm_resource_group.products.name
  location                 = azurerm_resource_group.products.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = var.ENVIROMENT
  }
}

resource "azurerm_storage_container" "products_website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "container"
}

resource "azurerm_storage_container" "docs" {
  name                  = "docs"
  storage_account_name  = azurerm_storage_account.products.name
  container_access_type = "blob"
}

# waiting for this to be resolved: https://github.com/terraform-providers/terraform-provider-azurerm/issues/1903
# (which is imminent), but in the meantime ...
module "products_staticweb" {
  source               = "git@github.com:StefanSchoof/terraform-azurerm-static-website.git"
  storage_account_name = azurerm_storage_account.products.name
}

resource "azurerm_search_service" "search" {
  name                = "mhraproductsnonprod"
  resource_group_name = azurerm_resource_group.products.name
  location            = azurerm_resource_group.products.location
  sku                 = "basic"

  tags = {
    environment = var.ENVIROMENT
  }
}

resource "azurerm_storage_account" "cpd" {
  name                     = "mhracpdnonprod"
  resource_group_name      = azurerm_resource_group.products.name
  location                 = azurerm_resource_group.products.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = var.ENVIROMENT
  }
}

resource "azurerm_storage_container" "cpd_website" {
  name                  = "$web"
  storage_account_name  = azurerm_storage_account.cpd.name
  container_access_type = "container"
}

# waiting for this to be resolved: https://github.com/terraform-providers/terraform-provider-azurerm/issues/1903
# (which is imminent), but in the meantime ...
module "cpd_staticweb" {
  source               = "git@github.com:StefanSchoof/terraform-azurerm-static-website.git"
  storage_account_name = azurerm_storage_account.cpd.name
}

# AKS

resource "azurerm_virtual_network" "network" {
  name                = "aks-vnet"
  location            = azurerm_resource_group.products.location
  resource_group_name = azurerm_resource_group.products.name
  address_space       = ["10.1.0.0/16"]
}

resource "azurerm_subnet" "subnet" {
  name                 = "aks-subnet"
  resource_group_name  = azurerm_resource_group.products.name
  address_prefix       = "10.1.0.0/24"
  virtual_network_name = azurerm_virtual_network.network.name
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = "aks"
  location            = azurerm_resource_group.products.location
  dns_prefix          = "aks"
  resource_group_name = azurerm_resource_group.products.name

  default_node_pool {
    name       = "products"
    node_count = 1
    vm_size    = "Standard_D2_v2"
  }

  service_principal {
    client_id     = var.CLIENT_ID
    client_secret = var.CLIENT_SECRET
  }


  tags = {
    Environment = var.ENVIROMENT
  }
}



resource "azurerm_container_registry" "container_registry" {
  name                = "mhraProductsNonProd"
  resource_group_name = azurerm_resource_group.products.name
  location            = azurerm_resource_group.products.location
  sku                 = "Basic"

  tags = {
    Environment = var.ENVIROMENT
  }
}
