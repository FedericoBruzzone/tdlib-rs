export RUST_BACKTRACE := 1

# Usage: make <command> ARGS="--features <feature> --bin <bin_name>"
#
# Avaialble commands:
#   all
#   build
#   run
#   test
#   clippy
#   fmt
#   clean
#
# Available features:
#   local-tdlib
#   download-tdlib
#   pkg-config
#
# Available bin_name:
#   get_me

all:
	$(MAKE) fmt
	$(MAKE) clippy # ARGS="--features download-tdlib"
	$(MAKE) test # ARGS="--features download-tdlib"

build_local:
	cargo build --no-default-features --features local-tdlib

# Example 1: make build ARGS="--features download-tdlib"
# Example 2: make build ARGS="--features download-tdlib --example telegram"
build:
	cargo build $(ARGS)

# Example: make run ARGS="--package tdlib-rs --example get_me --features download-tdlib"
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


