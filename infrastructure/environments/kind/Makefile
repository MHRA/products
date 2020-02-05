.PHONY: cluster install_zeebe

create_cluster:
	kind create cluster --config ./config.yaml

delete_cluster:
	kind delete cluster

install_zeebe:
	kubectl apply -k ./zeebe

delete_zeebe:
	kubectl delete -k ./zeebe
