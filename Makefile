build:
	docker build --target prod -t registry.home.ryougi.ca/simbru-pwa front
	docker push registry.home.ryougi.ca/simbru-pwa

helm-deploy:
	helm upgrade --install simbru-pwa helm --namespace simbru-pwa --create-namespace \
		--set image.repository=registry.home.ryougi.ca/simbru-pwa \
		--set image.tag=latest \
		--set ingress.enabled=true \
		--set ingress.className=nginx \
		--set ingress.hosts[0].host=simbru.home.ryougi.ca \
		--set ingress.hosts[0].paths[0].path=/ \
		--set ingress.hosts[0].paths[0].pathType=ImplementationSpecific \

build-no-cache:
	docker build --target prod -t registry.home.ryougi.ca/simbru-pwa front --no-cache
	docker push registry.home.ryougi.ca/simbru-pwa


deploy: build helm-deploy

decrypt:
	@echo "Decrypting secrets for deployment"
	sops -d front/env/.env.enc.production > front/env/.env.production
	@echo "Copy the decrypted file to .env.local for running locally"
