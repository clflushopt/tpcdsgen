use clap::Parser;
use tpcdsgen::config::{Options, Table};
use tpcdsgen::distribution::EnglishDistributions;
use tpcdsgen::random::RandomNumberStreamImpl;

fn main() {
    let options = Options::parse();
    
    match options.to_session() {
        Ok(session) => {
            println!("TPC-DS Data Generator (Rust implementation)");
            println!("Scale factor: {}", session.get_scaling().get_scale());
            println!("Target directory: {}", session.get_target_directory());
            println!("File suffix: {}", session.get_suffix());
            println!("Column separator: '{}'", session.get_separator());
            println!("Parallelism: {}", session.get_parallelism());
            
            if session.generate_only_one_table() {
                let table = session.get_only_table_to_generate();
                println!("Generating table: {} ({})", table.get_name(), table);
                println!("Estimated row count: {}", session.get_scaling().get_row_count(table));
            } else {
                println!("Generating all tables");
                let main_tables = Table::main_tables();
                println!("Main tables to generate: {}", main_tables.len());
                
                for table in main_tables.iter().take(5) {
                    println!("  {}: ~{} rows", 
                             table.get_name(), 
                             session.get_scaling().get_row_count(*table));
                }
                if main_tables.len() > 5 {
                    println!("  ... and {} more tables", main_tables.len() - 5);
                }
            }
            
            if !session.get_command_line_arguments().is_empty() {
                println!("Equivalent command line: tpcdsgen {}", session.get_command_line_arguments());
            }
            
            // Demo the distribution system
            println!("\n--- Distribution System Demo ---");
            let mut stream = RandomNumberStreamImpl::new(1).unwrap();
            
            println!("Random English words:");
            for i in 0..5 {
                let adjective = EnglishDistributions::pick_random_adjective(&mut stream).unwrap();
                let noun = EnglishDistributions::pick_random_noun(&mut stream).unwrap();
                println!("  {}. {} {}", i + 1, adjective, noun);
            }
            
            let phrase = EnglishDistributions::generate_random_phrase(&mut stream, 4).unwrap();
            println!("Random phrase: {}", phrase);
            
            println!("\nImplementation in progress...");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}