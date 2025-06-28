# Makefile pour le développement et la CI
SERVICE_NAME ?= rust-template-service
DOCKER_IMAGE ?= ghcr.io/${GITHUB_REPOSITORY_OWNER}/${SERVICE_NAME}

.PHONY: fmt lint test build docker-build docker-run dev

# Formattage
fmt:
	cargo fmt

# Linting strict
lint:
	cargo clippy -- -D warnings

test:
	cargo test -- --nocapture

# Construction binaire release
build:
	cargo build --release

# Build image Docker locale
docker-build:
	docker build -t $(DOCKER_IMAGE):latest .

# Exécuter le container Docker
docker-run:
	docker run --rm -p 8000:8000 $(DOCKER_IMAGE):latest

# Démarrage en dev via docker-compose
dev:
	devcontainer up --workspace-folder .
	devcontainer exec --workspace-folder . -- /bin/bash
