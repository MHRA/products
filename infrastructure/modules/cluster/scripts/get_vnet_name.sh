#!/bin/bash
set -e
resource_group_name=$1
az resource list --resource-group "${resource_group_name}" --resource-type Microsoft.Network/virtualNetworks | jq ".[0]"
