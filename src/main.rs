use std::env;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <comma-separated-symbols>", args[0]);
        eprintln!("Example: {} FIU.TO,GDV.TO", args[0]);
        return;
    }

    let symbols_arg = &args[1];
    let symbols: Vec<&str> = symbols_arg.split(',').map(|s| s.trim()).collect();

    let provider = yahoo::YahooConnector::new();

    match provider {
        Ok(provider) => {
            let mut results = Vec::new();
            let mut errors = Vec::new();

            for symbol in symbols {
                match provider.get_latest_quotes(symbol, "1d").await {
                    Ok(response) => match response.last_quote() {
                        Ok(quote) => {
                            results.push((symbol, quote.close));
                        }
                        Err(_) => {
                            errors.push(format!("No quote data found for {}", symbol));
                        }
                    },
                    Err(e) => {
                        errors.push(format!("Error fetching {}: {}", symbol, e));
                    }
                }
            }

            // Print errors first
            for error in errors {
                eprintln!("{}", error);
            }

            println!();
            println!("CSV data:");
            println!();
            // Then print CSV output
            println!("Symbol,Price");
            for (symbol, price) in results {
                println!("{},{:.2}", symbol, price);
            }
        }
        Err(e) => eprintln!("Failed to create Yahoo connector: {}", e),
    }
}
