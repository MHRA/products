provider "azurerm" {
  version = "~> 1.38.0"
  # subscription_id = "b2f4966f-71ea-415b-9ec6-bf96d597d596"
}

terraform {
  required_version = "0.12.18"

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
    environment = "non-prod"
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
    environment = "non-prod"
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
    environment = "non-prod"
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
    environment = "non-prod"
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




  tags = {
    Environment = "non-prod"
  }

}


output "client_certificate" {
  value = azurerm_kubernetes_cluster.cluster.kube_config.0.client_certificate
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config_raw
}
