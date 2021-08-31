resource "azuread_application" "pars-upload" {
  count                      = var.include_pars_app ? 1 : 0
  display_name               = "pars-upload-${var.environment}"
  reply_urls                 = concat(var.additional_allowed_pars_reply_urls, ["https://${azurerm_cdn_endpoint.pars.host_name}", "https://${azurerm_storage_account.pars.primary_web_host}"])
  group_membership_claims    = "SecurityGroup"
  oauth2_allow_implicit_flow = true
  owners                     = var.app_registration_owners
  app_role {
    allowed_member_types = [
      "User",
    ]
    description  = "used to secure the pars upload endpoint via JWT"
    display_name = "pars_upload"
    is_enabled   = false
    value        = "id_token"
  }

  required_resource_access {
    resource_app_id = "00000003-0000-0000-c000-000000000000"

    resource_access {
      id   = "e1fe6dd8-ba31-4d61-89e7-88639da4683d"
      type = "Scope"
    }
    resource_access {
      id   = "df021288-bdef-4463-88db-98f22de89214"
      type = "Role"
    }
  }
}
