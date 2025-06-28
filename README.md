# podia-filtering-service

**Rust microservice for image filtering in the POD + IA workflow.**

## Prerequisites
- Rust toolchain (stable)
- Docker & Docker Compose
- devcontainer-cli (optional, for containerized development)

## Installation
```bash
git clone git@github.com:your-org/podia-filtering-service.git
cd podia-filtering-service
cargo build --release

## Development

### Local (Dev Container)

``bash
make dev
```

### Local (Host)
```bash
cargo run
```

## Commands
- make lint: Run linting
- make test: Run test
- make build: Build Docker image
- make run: Run service
- make docs: Generate documentation
- make docs-serve: Serve documentation
- make dev: Run local development server
