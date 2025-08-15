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

**Note:** The just commands need to be updated to include the mandatory `--output` parameter.

```bash
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

Run the program with comma-separated stock symbols and a mandatory output file:

```bash
# Show help
cargo run -- --help

# Using cargo run with output file
cargo run -- AAPL,MSFT --output prices.csv

# Using the built binary
./target/release/stock-values FIU.TO,GDV.TO --output portfolio.csv
```

### Examples

```bash
# Get prices for Canadian ETFs
cargo run -- FIU.TO,GDV.TO,XIC.TO --output etfs.csv

# Single stock
cargo run -- SHOP.TO --output shopify.csv

# Multiple stocks with spaces (will be trimmed)
cargo run -- "FIU.TO, GDV.TO, XIC.TO" --output portfolio.csv

# Using short form of output flag
cargo run -- AAPL,MSFT -o tech_stocks.csv
```

## Output

The program writes CSV format to the specified file with any errors shown to stderr:

**File content (e.g., portfolio.csv):**
```
Symbol,Price
FIU.TO,25.43
GDV.TO,18.72
XIC.TO,32.15
```

**Console output:**
```
CSV data written to portfolio.csv
```

## Dependencies

- **yahoo_finance_api**: Rust client for Yahoo Finance API
- **tokio**: Async runtime
- **clap**: Command-line argument parsing with help generation

## Features

- ✅ **Free access** to stock quotes worldwide
- ✅ **CSV file output** - writes data to specified file
- ✅ **Command-line help** - built-in help and version information
- ✅ **Error handling** - shows errors to stderr, clean CSV to file
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
