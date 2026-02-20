.PHONY: test test-rust test-e2e test-quick build clean help setup-hooks web-dev web-build web-type-check

# Default target
help:
	@echo "Available commands:"
	@echo "  make help        - Show this help message"
	@echo "  make setup-hooks - Configure git hooks (run after clone)"
	@echo "  make test        - Run ALL tests (Rust + E2E)"
	@echo "  make test-rust   - Run Rust tests only (core + cli)"
	@echo "  make test-e2e    - Run E2E tests only (GUI)"
	@echo "  make test-quick  - Run quick tests (Rust only, no E2E)"
	@echo "  make build       - Build all components"
	@echo "  make install     - Install all dependencies"
	@echo "  make run         - Run the Tauri application (Dev)"
	@echo "  make web-dev     - Run the Next.js web app (Dev)"
	@echo "  make web-build   - Build the Next.js web app"
	@echo "  make clean       - Clean build artifacts"

# Install dependencies
install:
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Installing GUI dependencies..."
	cd gui && pnpm install
	@echo "✓ Dependencies installed"

# Run the Tauri application (Dev mode)
run:
	@echo "Starting Tauri app..."
	cd gui && pnpm tauri dev

# Setup git hooks (run this after cloning)
setup-hooks:
	@echo "Configuring git hooks..."
	git config core.hooksPath .githooks
	@echo "✓ Git hooks configured. Pre-push will run all tests."

# Run all tests
test: test-rust test-e2e
	@echo "✓ All tests completed"

# Rust tests (core + cli + gui-tauri)
test-rust:
	@echo "Running Rust tests..."
	cargo test --workspace
	@echo "✓ Rust tests completed"

# E2E tests (GUI with Playwright)
test-e2e:
	@echo "Running E2E tests..."
	cd gui/tests && pnpm test
	@echo "✓ E2E tests completed"

# Quick tests (skip slow E2E)
test-quick: test-rust
	@echo "✓ Quick tests completed (E2E skipped)"

# Build all
build:
	@echo "Building Rust workspace..."
	cargo build --workspace
	@echo "Building GUI..."
	cd gui && pnpm build
	@echo "✓ Build completed"

# Web app (Next.js)
web-dev:
	@echo "Starting Next.js dev server..."
	cd web && pnpm dev

web-build:
	@echo "Building Next.js app..."
	cd web && pnpm build

web-type-check:
	@echo "Type checking web app..."
	cd web && pnpm type-check

# Clean
clean:
	cargo clean
	rm -rf gui/dist
	rm -rf web/.next
	@echo "✓ Cleaned"
