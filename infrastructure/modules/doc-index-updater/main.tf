locals {
  name = "doc-index-updater"
}


## Cluster
resource "azurerm_public_ip" "doc_index_updater_ip" {
  name                = "${local.name}-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_kubernetes_cluster" "doc_index_updater_cluster" {
  name                = "${local.name}-cluster"
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name       = local.name
    node_count = 2
    vm_size    = "Standard_D2_v2"
  }


  service_principal {
    client_id     = var.client_id
    client_secret = var.client_secret
  }

  network_profile {
    network_plugin = "kubenet"
  }



  tags = {
    Environment = var.environment
  }
}

