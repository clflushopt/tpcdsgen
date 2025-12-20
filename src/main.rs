use clap::Parser;
use tpcdsgen::config::{Options, Table};

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
                println!("Row count: {}", session.get_scaling().get_row_count(table));
            } else {
                println!("Generating all tables");
                let main_tables = Table::main_tables();
                println!("Main tables to generate: {}", main_tables.len());

                for table in main_tables.iter().take(5) {
                    println!(
                        "  {}: ~{} rows",
                        table.get_name(),
                        session.get_scaling().get_row_count(*table)
                    );
                }
                if main_tables.len() > 5 {
                    println!("  ... and {} more tables", main_tables.len() - 5);
                }
            }

            if !session.get_command_line_arguments().is_empty() {
                println!(
                    "Equivalent command line: tpcdsgen {}",
                    session.get_command_line_arguments()
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
