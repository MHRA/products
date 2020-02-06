provider "azurerm" {
  version = "~> 1.38.0"
}

terraform {
  backend "azurerm" {
    resource_group_name  = "tfstate"
    container_name       = "tfstate"
    storage_account_name = "tfstate2337"
    key                  = "dev.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "mdr" {
  name     = var.RESOURCE_GROUP_NAME
  location = var.REGION
}

# AKS
module cluster {
  source = "../../modules/cluster"

  environment         = var.ENVIRONMENT
  location            = var.REGION
  resource_group_name = azurerm_resource_group.mdr.name
  node_count          = 3

  client_id     = var.CLIENT_ID
  client_secret = var.CLIENT_SECRET
}
