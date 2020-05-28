resource "azuread_application" "pars-upload" {
  name                    = "pars-upload-${var.environment}"
  reply_urls              = concat(["https://${azurerm_cdn_endpoint.pars.host_name}", "https://${azurerm_storage_account.pars.primary_web_host}", "https://mhraparsnonprod.z33.web.core.windows.net", "https://mhraparsnon-prod.azureedge.net"], var.add_local_pars_reply_url ? ["http://localhost:3000"] : [])
  group_membership_claims = "SecurityGroup"
  oauth2_allow_implicit_flow = true
  owners = var.app_registration_owners
  app_role {
    allowed_member_types = [
      "User",
    ]
    description  = "used to secure the pars upload endpoint via JWT"
    display_name = "pars_upload"
    is_enabled   = false
    value        = "id_token"
  }
}
