resource "azurerm_network_security_group" "lb_subnet" {
  name                = var.lb_subnet_name
  location            = var.location
  resource_group_name = var.resource_group_name

  security_rule {                                                                                                                                                                                             
     name                       = "DenyAzureLoadBalancerInBound"                                                                                                                                                                  
     priority                   = 100                                                                                                                                                                          
     direction                  = "Inbound"                                                                                                                                                                    
     access                     = "Deny"                                                                                                                                                                      
     protocol                   = "*"                                                                                                                                                                        
     source_port_range          = "*"                                                                                                                                                                          
     destination_port_range     = "*"                                                                                                                                                                      
     source_address_prefix      = "AzureLoadBalancer"                                                                                                                                                                   
     destination_address_prefix = "*"
   }    

    security_rule {                                                                                                                                                                                             
     name                       = "DenyInternetOutBound"                                                                                                                                                                  
     priority                   = 100                                                                                                                                                                          
     direction                  = "Outbound"                                                                                                                                                                    
     access                     = "Deny"                                                                                                                                                                      
     protocol                   = "*"                                                                                                                                                                        
     source_port_range          = "*"                                                                                                                                                                          
     destination_port_range     = "*"                                                                                                                                                                      
     source_address_prefix      = "*"                                                                                                                                                                   
     destination_address_prefix = "Internet"
   }  

  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet_network_security_group_association" "lb_subnet" {
  subnet_id                 = var.lb_subnet_id
  network_security_group_id = azurerm_network_security_group.lb_subnet.id
}
