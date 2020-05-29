resource "azuread_application" "pars-upload" {
  name                       = "pars-upload-${var.environment}"
  reply_urls                 = concat(var.addtional_allowed_pars_reply_urls, ["https://${azurerm_cdn_endpoint.pars.host_name}", "https://${azurerm_storage_account.pars.primary_web_host}"])
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
}
