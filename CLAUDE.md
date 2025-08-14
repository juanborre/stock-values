# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust command-line tool for fetching stock prices (including Canadian ETFs) using the Yahoo Finance API.

## 🏗️ Project Architecture

### Core Design
- **Language**: Rust (2024 edition)
- **Runtime**: Async using Tokio
- **API**: Yahoo Finance (no API key required)
- **Output Format**: CSV with error handling to stderr
- **Build System**: Cargo with Just task runner
- **Cross-platform**: Native support for Linux/macOS, Docker-based Windows builds

### Key Dependencies
```toml
tokio = { version = "1.0", features = ["full"] }    # Async runtime
yahoo_finance_api = "2.0"                           # Yahoo Finance client  
dotenvy = "0.15"                                     # Environment variable loading (unused in main code)
```

## 📁 Project Structure

```
stock-values/
├── src/
│   └── main.rs              # Single-file application logic
├── .github/
│   └── workflows/
│       └── build.yml        # Windows CI/CD build pipeline
├── target/                  # Cargo build artifacts
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── justfile                # Task runner commands
├── README.md               # User documentation
├── CLAUDE.md               # Claude Code guidance file
└── .gitignore             # Git ignore patterns
```

## 🔧 Core Functionality

### Main Application Flow (`/Users/juan/transit/stock-values/src/main.rs`)
1. **Command Line Parsing**: Accepts comma-separated stock symbols as arguments
2. **Yahoo Finance Connection**: Creates connector without requiring API keys
3. **Sequential Data Fetching**: Iterates through symbols, fetches latest quotes one by one
4. **Error Handling**: Separates successful results from errors
5. **Output Generation**: Prints errors to stderr first, then CSV data to stdout

### Key Features
- ✅ **Free API Access**: Uses Yahoo Finance without authentication
- ✅ **Multi-symbol Support**: Batch processing of stock symbols
- ✅ **Error Isolation**: Clean CSV output with errors sent to stderr
- ✅ **International Markets**: US, Canadian (.TO), and other global markets
- ✅ **Real-time Data**: Latest closing prices
- ✅ **Cross-platform**: Windows, macOS, Linux support

### Input/Output Format
```bash
# Input
./stocks-values "AAPL,MSFT,SHOP.TO"

# Errors to stderr (if any) 
Error fetching INVALID: Invalid symbol

# Output to stdout

CSV data:

Symbol,Price
AAPL,175.43
MSFT,420.15
SHOP.TO,85.32
```

## 🛠️ Development Workflow

### Just Task Runner Commands (`/Users/juan/transit/stock-values/justfile`)
```bash
# Core development commands
just run "AAPL,MSFT"        # Run with symbols
just check                  # Check code without building
just clean                  # Clean build artifacts
just help                   # Show all commands (just --list)

# Cross-compilation
just build-windows          # Docker-based Windows build using rust:1.85 container
just package-windows        # Package Windows binary to dist/

# Release management  
just tag 1.0.0              # Tag version (triggers GitHub Actions)
just github-status          # Show GitHub Actions URL
```

### Standard Cargo Commands
```bash
cargo run -- "AAPL,MSFT"   # Direct cargo execution
cargo check                 # Fast syntax/type checking
cargo build --release       # Optimized production build
cargo clean                 # Clean build artifacts
```

### No Testing Infrastructure
- No unit tests detected in the codebase
- No test configuration in Cargo.toml
- No testing commands in justfile
- Consider adding tests for error handling and API integration

## 🔄 CI/CD Pipeline

### GitHub Actions (`/Users/juan/transit/stocks-values/.github/workflows/build.yml`)
- **Trigger**: Git tags matching `v*` pattern or manual dispatch
- **Platform**: Windows-latest runner
- **Target**: `x86_64-pc-windows-msvc`
- **Output**: Compiled `.exe` binary as GitHub Actions artifact
- **Toolchain**: Latest stable Rust

### Cross-compilation Strategy
1. **Local Development**: Native builds on macOS/Linux
2. **Windows Distribution**: 
   - GitHub Actions (recommended): Tag-triggered builds
   - Docker Alternative: Local cross-compilation using `rust:1.85` container

## 🗂️ Configuration Files

### Build Configuration
- **Cargo.toml**: Standard Rust 2024 edition project
- **Cargo.lock**: Locked dependency versions for reproducible builds
- **No custom linting**: No clippy.toml or rustfmt.toml detected

### Environment Configuration
- **dotenvy dependency**: Included but not utilized in main application
- No environment files detected in current codebase

### Git Configuration
- **.gitignore**: Standard Rust patterns plus custom exclusions:
  - Build artifacts (`target/`, `debug/`)
  - IDE files (RustRover, JetBrains)
  - Distribution files (`dist/`)
  - Claude integration files (`.claude/`)
  - Backup files (`**/*.rs.bk`)
  - Windows debug files (`*.pdb`)
  - Mutation testing data (`**/mutants.out*/`)

## 🔍 Code Quality & Patterns

### Architecture Patterns
- **Single Responsibility**: One focused binary for stock price fetching
- **Error Handling**: Comprehensive error collection and reporting
- **Separation of Concerns**: Errors to stderr, data to stdout
- **Async/Await**: Proper use of Tokio async runtime

### Potential Improvements
1. **Testing**: Add unit tests for error scenarios and API mocking
2. **Configuration**: Utilize the loaded dotenvy for optional settings
3. **Logging**: Consider structured logging for better debugging
4. **Caching**: Add optional caching for repeated symbol requests
5. **Validation**: Input validation for stock symbol formats
6. **Concurrency**: Parallel API requests instead of sequential processing

### Security Considerations
- **Input Sanitization**: Basic symbol parsing without validation
- **Network Security**: Relies on yahoo_finance_api crate for HTTPS

## 🚀 Deployment & Distribution

### Local Usage
```bash
# Development
just run "AAPL,MSFT,SHOP.TO"

# Production
cargo build --release
./target/release/stocks-values "AAPL,MSFT,SHOP.TO"
```

### Cross-platform Distribution
1. **Windows**: GitHub Actions artifacts or `just build-windows`
2. **macOS/Linux**: Native `cargo build --release`
3. **Binary Size**: Single executable, no external dependencies

### Installation Requirements
- **Rust**: 2024 edition compatibility
- **Just**: Task runner (`brew install just`)
- **Docker**: For Windows cross-compilation (optional)

## 📊 Technical Specifications

### Performance Characteristics
- **Memory**: Minimal heap usage, vector-based result collection
- **Network**: Sequential API calls (room for parallelization improvement)
- **CPU**: Lightweight JSON parsing and CSV formatting
- **Dependencies**: Minimal dependency footprint with tokio and yahoo_finance_api

### Supported Markets
- 🇺🇸 US: NYSE, NASDAQ (AAPL, MSFT, TSLA)
- 🇨🇦 Canadian: TSX with .TO suffix (SHOP.TO, FIU.TO)
- 🌍 International: Various global exchanges supported by Yahoo Finance

### Data Format
- **Input**: Comma-separated stock symbols with optional whitespace
- **Output**: Standard CSV with Symbol,Price headers
- **Precision**: 2 decimal places for prices
- **Encoding**: UTF-8 text output

This codebase represents a focused, well-structured tool for stock price fetching with good cross-platform support and a clear development workflow using modern Rust practices.