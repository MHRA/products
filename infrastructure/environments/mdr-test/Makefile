.PHONY: default init init-reconfigure

default:
	terraform apply

init:
	terraform init --backend-config "storage_account_name=${TF_VAR_STORAGE_ACCOUNT_NAME}" --backend-config "access_key=${TF_VAR_ACCESS_KEY}" --backend-config "container_name=${TF_VAR_CONTAINER_NAME}"

init-reconfigure:
	terraform init --backend-config "storage_account_name=${TF_VAR_STORAGE_ACCOUNT_NAME}" --backend-config "access_key=${TF_VAR_ACCESS_KEY}" --backend-config "container_name=${TF_VAR_CONTAINER_NAME}" --reconfigure

