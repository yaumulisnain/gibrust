.PHONY: build run test clean check fmt lint help

# Default target when just running 'make'
.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo "  build      - Build the project in debug mode"
	@echo "  build-rel  - Build the project in release mode"
	@echo "  run        - Run the project in debug mode"
	@echo "  run-rel    - Run the project in release mode"
	@echo "  test       - Run all tests"
	@echo "  clean      - Remove build artifacts"
	@echo "  check      - Run cargo check"
	@echo "  fmt        - Format code using rustfmt"
	@echo "  lint       - Run clippy lints"
	@echo "  watch      - Watch for changes and rebuild (requires cargo-watch)"

# Build targets
build:
	cargo build

build-rel:
	cargo build --release

# Run targets
run:
	cargo run

run-rel:
	cargo run --release

# Development targets
test:
	cargo test

clean:
	cargo clean

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x 'run'

# Install development dependencies
dev-deps:
	cargo install cargo-watch
	cargo install cargo-edit
	cargo install cargo-update
