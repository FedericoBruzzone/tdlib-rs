# vim: set ft=make :
set windows-powershell := true
export RUST_BACKTRACE := "1"
project_name := "tdlib-rs"

_default:
  just --list --justfile {{justfile()}}

# All
all: fmt clippy test build

# Build the project using cargo
build:
  cargo build

# Run the project using cargo
# The BIN_FLAG is the name of the folder where the binary is located
#
# Example:
#  just run --package tdlib-rs --example get_me
run PACKAGE_FLAG="" PACKAGE_NAME="" BIN_FLAG="" BIN_NAME="":
  cargo run {{PACKAGE_FLAG}} {{PACKAGE_NAME}} {{BIN_FLAG}} {{BIN_NAME}}

# Format the code using cargo
fmt:
  cargo fmt --all
  cargo fmt --all -- --check

# Format the code using cargo nightly
fmt_nightly:
  cargo +nightly fmt --all
  cargo +nightly fmt --all -- --check

# Run clippy using cargo
clippy:
  cargo clippy --all-targets -- -D warnings

# Run tests using cargo
test:
  cargo test -- --nocapture --test-threads=1

# Clean the project using cargo
clean:
  cargo clean

_help:
  @echo "Usage: just [recipe]"
  @echo ""
  @echo "Available recipes:"
  @echo "  build       # Build the project using cargo"
  @echo "  run         # Run the project using cargo"
  @echo "  fmt_nightly # Format the code using cargo nightly"
  @echo "  fmt         # Format the code using cargo"
  @echo "  clippy      # Run clippy using cargo"
  @echo "  test        # Run tests using cargo"
  @echo "  clean       # Clean the project using cargo"
  @echo "  help        # Display this help message"

