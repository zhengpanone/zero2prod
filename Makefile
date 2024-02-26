.PHONY: help dev build test clean db-up db-down migrate migrate-revert docker-build docker-up docker-down fmt check

# Default target
help:
	@echo "Available commands:"
	@echo "  make dev           - Run the application in development mode"
	@echo "  make build         - Build the application in release mode"
	@echo "  make test          - Run tests"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make db-up         - Start PostgreSQL database"
	@echo "  make db-down       - Stop PostgreSQL database"
	@echo "  make migrate       - Run database migrations"
	@echo "  make migrate-revert - Revert last migration"
	@echo "  make docker-build  - Build Docker image"
	@echo "  make docker-up     - Start all services with Docker Compose"
	@echo "  make docker-down   - Stop all services"
	@echo "  make fmt           - Format code"
	@echo "  make check         - Run cargo check and clippy"

# Development
dev:
	cargo watch -x run

# Build
build:
	cargo build --release

# Test
test:
	cargo test

# Clean
clean:
	cargo clean

# Database - local
db-up:
	docker-compose up -d postgres

db-down:
	docker-compose down postgres

# Migrations
migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

# Create new migration
migrate-create:
	@read -p "Enter migration name: " name; \
	sqlx migrate add $$name

# Docker
docker-build:
	docker build -t zero2prod:latest .

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f api

# Code quality
fmt:
	cargo fmt

check:
	cargo check
	cargo clippy -- -D warnings

# Install development dependencies
install-dev:
	cargo install cargo-watch
	cargo install sqlx-cli --no-default-features --features postgres

# Quick start for new developers
setup: install-dev
	cp .env.example .env
	@echo "Please update .env with your configuration"
	make db-up
	sleep 3
	make migrate
	@echo "Setup complete! Run 'make dev' to start the server"