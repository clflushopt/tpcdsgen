use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{CallCenterRowGenerator, RowGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let num_rows = if args.len() > 1 {
        args[1].parse::<i64>().unwrap_or(6)
    } else {
        6
    };

    let output_file = if args.len() > 2 {
        args[2].clone()
    } else {
        "call_center.dat".to_string()
    };

    // Create a session with default settings
    let session = Session::get_default_session();

    // Create the call center row generator
    let mut generator = CallCenterRowGenerator::new();

    // Create output file
    let output_path = Path::new(&output_file);
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    println!(
        "Generating {} call center rows to '{}'...",
        num_rows, output_file
    );

    // Generate rows
    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        // Get the generated rows
        let rows = result.get_rows();

        for row in rows {
            let values = row.get_values();

            // Write CSV row (pipe-delimited as per TPC-DS standard)
            let csv_line = values.join("|");
            writeln!(writer, "{}", csv_line)?;

            // Print progress for larger datasets
            if row_number <= 5 || row_number % 1000 == 0 {
                println!(
                    "Generated row {}: {} characters",
                    row_number,
                    csv_line.len()
                );
            }
        }
    }

    writer.flush()?;

    println!(
        "✓ Generated call center data written to: {}",
        output_path.display()
    );
    println!("✓ File contains {} rows", num_rows);

    // Show file size
    let metadata = std::fs::metadata(output_path)?;
    println!("✓ File size: {} bytes", metadata.len());

    // Show first few lines
    if num_rows <= 20 {
        println!("\nGenerated data:");
        let content = std::fs::read_to_string(output_path)?;
        for (i, line) in content.lines().enumerate() {
            if i < 3 {
                println!("Row {}: {}", i + 1, line);
            }
        }
        if num_rows > 3 {
            println!("... ({} more rows)", num_rows - 3);
        }
    }

    Ok(())
}
