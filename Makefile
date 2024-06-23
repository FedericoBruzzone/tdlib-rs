export RUST_BACKTRACE := 1


all: fmt clippy test build

build:
    cargo build $(ARGS)

# Example: make run ARGS="--package tdlib-rs --example get_me"
run:
	cargo run $(ARGS)

fmt:
	cargo fmt --all
	cargo fmt --all -- --check

fmt_nightly:
	cargo +nightly fmt --all
	cargo +nightly fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test --verbose --workspace --exclude tdlib-rs -- --nocapture --test-threads=1

clean:
	cargo clean

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@echo "  build       # Build the project using cargo"
	@echo "  run         # Run the project using cargo"
	@echo "  fmt         # Format the code using cargo"
	@echo "  fmt_nightly # Format the code using nightly cargo"
	@echo "  clippy      # Run clippy using cargo"
	@echo "  test        # Run tests using cargo"
	@echo "  clean       # Clean the project using cargo"
	@echo "  help        # Display this help message"

# Each entry of .PHONY is a target that is not a file
.PHONY: build run test clean


