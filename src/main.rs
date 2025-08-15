use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use yahoo_finance_api as yahoo;

#[derive(Parser)]
#[command(name = "stock-values")]
#[command(about = "A Rust command-line tool for fetching stock prices using Yahoo Finance API")]
#[command(version = "0.1.0")]
struct Args {
    #[arg(help = "Comma-separated list of stock symbols (e.g., AAPL,MSFT,SHOP.TO)")]
    symbols: String,
    
    #[arg(short, long, help = "Output CSV file path (mandatory)")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let symbols: Vec<&str> = args.symbols.split(',').map(|s| s.trim()).collect();

    let provider = yahoo::YahooConnector::new();

    match provider {
        Ok(provider) => {
            let provider = Arc::new(provider);
            let mut handles = Vec::new();

            // Create concurrent tasks for each symbol
            for symbol in symbols {
                let provider_clone = Arc::clone(&provider);
                let symbol_owned = symbol.to_string();
                
                let handle = tokio::spawn(async move {
                    match provider_clone.get_latest_quotes(&symbol_owned, "1d").await {
                        Ok(response) => match response.last_quote() {
                            Ok(quote) => Ok((symbol_owned, quote.close)),
                            Err(_) => Err(format!("No quote data found for {}", symbol_owned)),
                        },
                        Err(e) => Err(format!("Error fetching {}: {}", symbol_owned, e)),
                    }
                });
                handles.push(handle);
            }

            // Wait for all tasks to complete
            let mut results = Vec::new();
            let mut errors = Vec::new();

            for handle in handles {
                match handle.await {
                    Ok(task_result) => match task_result {
                        Ok((symbol, price)) => results.push((symbol, price)),
                        Err(error_msg) => errors.push(error_msg),
                    },
                    Err(e) => errors.push(format!("Task error: {}", e)),
                }
            }

            // Print errors to stderr
            for error in errors {
                eprintln!("{}", error);
            }

            // Write CSV output to file
            let mut csv_content = String::new();
            csv_content.push_str("Symbol,Price\n");
            for (symbol, price) in results {
                csv_content.push_str(&format!("{},{:.2}\n", symbol, price));
            }

            match File::create(&args.output) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(csv_content.as_bytes()) {
                        eprintln!("Error writing to file {}: {}", args.output, e);
                    } else {
                        println!("CSV data written to {}", args.output);
                    }
                }
                Err(e) => eprintln!("Error creating file {}: {}", args.output, e),
            }
        }
        Err(e) => eprintln!("Failed to create Yahoo connector: {}", e),
    }
}
