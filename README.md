# stock-values

A Rust command-line tool for fetching stock prices (including Canadian ETFs) using the Yahoo Finance API.

## Setup

1. **Install Just** (task runner):
   ```bash
   # macOS
   brew install just

   # Other platforms: https://github.com/casey/just
   ```

2. **Build the project**:
   ```bash
   just build
   ```

No API key required - Yahoo Finance provides free access to stock quotes!

## Usage with Just

```bash
# Run with your own stock symbols
just run "AAPL,MSFT,SHOP.TO"

# Run with a large portfolio
just run "AX-UN.TO,BANK.TO,BEP-UN.TO,BOND.TO,CAE.TO,CASH.TO"

# Check code without building
just check

# Clean build artifacts
just clean

# Tag a version (triggers Windows build)
just tag 1.0.0

# Show all available commands
just help
```

### Direct Usage

Run the program with comma-separated stock symbols:

```bash
# Using cargo run
cargo run -- FIU.TO,GDV.TO

# Using the built binary
./target/release/stock-values FIU.TO,GDV.TO
```

### Examples

```bash
# Get prices for Canadian ETFs
cargo run -- FIU.TO,GDV.TO,XIC.TO

# Single stock
cargo run -- SHOP.TO

# Multiple stocks with spaces (will be trimmed)
cargo run -- "FIU.TO, GDV.TO, XIC.TO"
```

## Output

The program outputs CSV format with any errors shown first:

```
Symbol,Price
FIU.TO,25.43
GDV.TO,18.72
XIC.TO,32.15
```

## Dependencies

- **yahoo_finance_api**: Rust client for Yahoo Finance API
- **tokio**: Async runtime

## Features

- ✅ **Free access** to stock quotes worldwide
- ✅ **CSV output format** - easy to pipe to files or other tools
- ✅ **Error handling** - shows errors to stderr, clean CSV to stdout
- ✅ **Canadian stocks** supported (TSX, .TO suffix)
- ✅ **US stocks** supported (NYSE, NASDAQ, AMEX)
- ✅ **Real-time pricing** data
- ✅ **No API key required**

### Supported Markets
- 🇺🇸 US stocks: AAPL, MSFT, TSLA, etc.
- 🇨🇦 Canadian stocks: SHOP.TO, FIU.TO, GDV.TO, XIC.TO, etc.
- 🌍 Many international markets supported

## Building Windows Binary

### GitHub Actions (Recommended)
1. Tag a version: `just tag 1.0.0`
2. GitHub Actions automatically builds Windows binary
3. Download from: `https://github.com/YOUR_USERNAME/stock-values/actions`

### Local Docker Build (Alternative)
```bash
just build-windows
just package-windows
```
