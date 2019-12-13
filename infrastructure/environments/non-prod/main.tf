provider "azurerm" {
  version = "=1.38.0"
}

terraform {
  required_version = "0.12.18"

  backend "azurerm" {
    resource_group_name = "tfstate"
    key                 = "non-prod.terraform.tfstate"
  }
}

resource "azurerm_resource_group" "rg" {
  name     = var.RESOURCE_GROUP
  location = var.REGION
}

