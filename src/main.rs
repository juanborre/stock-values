use std::env;
use std::sync::Arc;
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
