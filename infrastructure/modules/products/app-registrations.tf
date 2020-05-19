resource "azuread_application" "pars-upload" {
  name                    = "pars-upload-${var.namespace}"
  reply_urls              = ["http://${var.namespace}"] #don't think we need this
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
