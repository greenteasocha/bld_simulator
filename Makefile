.PHONY: setup wasm build run test fmt check clean deploy help

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Set up Rust environment for WASM development
	source ~/.cargo/env && \
	rustup default stable && \
	rustup target add wasm32-unknown-unknown

wasm: ## Build Rust to WebAssembly
	wasm-pack build --target web --out-dir pkg

build: ## Build native Rust application
	cargo build --release

run: ## Run TUI application
	cargo run

test: ## Run tests
	cargo test

fmt: ## Format code
	cargo fmt

check: ## Run lints and checks
	cargo clippy && cargo fmt --check

clean: ## Clean build artifacts
	cargo clean && rm -rf pkg/

dev: wasm ## Quick development build (WASM only)

all: setup wasm build ## Set up environment and build everything

deploy: wasm ## Deploy to AWS (requires certificateArn)
	@if [ -z "$(CERT_ARN)" ]; then \
		echo "Error: CERT_ARN is required. Usage: make deploy CERT_ARN=arn:aws:acm:..."; \
		exit 1; \
	fi
	cd infra && npm run deploy -- \
		-c domainName=cube.sochadev.click \
		-c hostedZoneName=sochadev.click \
		-c hostedZoneId=Z00143531ZEESXAZBGYA7 \
		-c certificateArn=$(CERT_ARN)