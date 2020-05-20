resource "azuread_application" "pars-upload" {
  name                    = "pars-upload-${var.environment}"
  reply_urls              = concat(["https://${azurerm_cdn_endpoint.pars.host_name}"], var.add_local_pars_reply_url ? ["http://localhost:3000"] : [])
  group_membership_claims = "All"
  homepage                = "http://pars-upload-${var.namespace}" #don't think we need this

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
