# Run with custom stock symbols and output file
run symbols output="stock-prices.csv":
    cargo run -- "{{symbols}}" --output "{{output}}"

# Show help
show-help:
    cargo run -- --help

# Clean build artifacts
clean:
    cargo clean

# Check code without building
check:
    cargo check

# Build for Windows using Docker (requires Docker Desktop)
build-windows:
    #!/usr/bin/env bash
    docker run --rm \
        -v "$(pwd)":/workspace \
        -w /workspace \
        rust:1.85 \
        bash -c "
            rustup target add x86_64-pc-windows-gnu && \
            apt-get update && \
            apt-get install -y gcc-mingw-w64-x86-64 && \
            cargo build --release --target x86_64-pc-windows-gnu
        "

# Package Windows binary (run after build-windows)
package-windows:
    mkdir -p dist
    cp target/x86_64-pc-windows-gnu/release/stock-values.exe dist/
    echo "Windows binary created at: dist/stock-values.exe"
    echo "You can send this file to your friend!"

# Tag a new version (triggers GitHub Actions build)
tag version:
    git tag v{{version}}
    git push origin v{{version}}
    @echo "Tagged version v{{version}} - check GitHub Actions for Windows build"

# Show GitHub Actions status (if repo is on GitHub)
github-status:
    @echo "Check GitHub Actions at: https://github.com/YOUR_USERNAME/stock-values/actions"
    @echo "Download Windows binary from the Actions artifacts"

# Show help
help:
    @just --list
