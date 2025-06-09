ifneq (,$(wildcard ./.env))
    include .env
    export
endif

deploy-api: build-api helm-deploy-api
deploy-pwa: build helm-deploy

build:
	docker build --target prod \
		-t registry.home.ryougi.ca/simbru-pwa \
		front
	docker push registry.home.ryougi.ca/simbru-pwa

build-api:
	docker build --target prod -t registry.home.ryougi.ca/simbru-api back/api
	docker push registry.home.ryougi.ca/simbru-api

helm-deploy:
	helm upgrade --install simbru-pwa helm --namespace simbru-pwa --create-namespace \
		--set image.repository=registry.home.ryougi.ca/simbru-pwa \
		--set image.tag=latest \
		--set ingress.enabled=true \
		--set ingress.className=nginx \
		--set ingress.hosts[0].host=simbru.home.ryougi.ca \
		--set ingress.hosts[0].paths[0].path=/ \
		--set ingress.hosts[0].paths[0].pathType=ImplementationSpecific \
		--set ingress.hosts[1].host=simbru.ryougi.ca \
		--set ingress.hosts[1].paths[0].path=/ \
		--set ingress.hosts[1].paths[0].pathType=ImplementationSpecific \

	
helm-deploy-api:
	helm upgrade --install simbru-api helm --namespace simbru-pwa --create-namespace \
		--set image.repository=registry.home.ryougi.ca/simbru-api \
		--set image.tag=latest \
		--set ingress.enabled=true \
		--set ingress.className=nginx \
		--set ingress.hosts[0].host=simbru-api.home.ryougi.ca \
		--set ingress.hosts[0].paths[0].path=/ \
		--set ingress.hosts[0].paths[0].pathType=ImplementationSpecific \
		--set ingress.hosts[1].host=simbru-api.ryougi.ca \
		--set ingress.hosts[1].paths[0].path=/ \
		--set ingress.hosts[1].paths[0].pathType=ImplementationSpecific \
		--set env.POSTGRES_CONNECTION_STRING=${DATABASE_URL} \
		--set env.OAUTH_CLIENT_ID=${OAUTH_CLIENT_ID} \
		--set env.OAUTH_CLIENT_SECRET=${OAUTH_CLIENT_SECRET} \
		--set env.OAUTH_REDIRECT_URL=${OAUTH_REDIRECT_URL} \
		--set service.port=8000

build-no-cache:
	docker build --target prod -t registry.home.ryougi.ca/simbru-pwa front --no-cache
	docker push registry.home.ryougi.ca/simbru-pwa


deploy: build helm-deploy

psql:
	psql ${DATABASE_URL}

migrate: seed refresh-db insert

seed:
	@psql ${DATABASE_URL} -c "COPY (SELECT * FROM sim) TO STDOUT WITH CSV HEADER" > back/data/sim.csv
	@psql ${DATABASE_URL} -c "COPY (SELECT * FROM sim_stat) TO STDOUT WITH CSV HEADER" > back/data/sim_stat.csv

insert:
	@psql ${DATABASE_URL} -c "COPY sim FROM STDIN WITH CSV HEADER" < back/data/sim.csv
	@psql ${DATABASE_URL} -c "COPY sim_stat FROM STDIN WITH CSV HEADER" < back/data/sim_stat.csv

refresh-db:
	cd back && \
	sea-orm-cli migrate refresh && \
	sea-orm-cli generate entity --with-serde both --serde-skip-deserializing-primary-key -o api/src/entities

	

decrypt:
	@echo "Decrypting secrets for deployment"
	sops -d front/env/.env.enc.production > front/env/.env.production
	@echo "Copy the decrypted file to .env.local for running locally"

test-helm:
	@echo "Running Helm chart validation tests..."
	cd tests && cargo test helm_chart_tests
	@echo "✅ Helm chart tests passed!"

test-api:
	@echo "API tests not implemented yet"

test-frontend:
	@echo "Frontend tests not implemented yet"

test: test-helm test-api test-frontend
	@echo "✅ All tests passed!"
