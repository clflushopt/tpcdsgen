use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{ReasonRowGenerator, RowGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();

    let mut generator = ReasonRowGenerator::new();

    let output_path = Path::new("reason.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let num_rows = 35; // Scale 1 has 35 rows per specification

    println!("Generating {} reason rows...", num_rows);

    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();

        for row in rows {
            let values = row.get_values();

            let csv_line = values.join("|");
            writeln!(writer, "{}|", csv_line)?;

            if row_number <= 10 {
                println!("Row {}: {}", row_number, csv_line);
            }
        }
    }

    writer.flush()?;
    println!(
        "Generated reason data written to: {}",
        output_path.display()
    );
    println!("File contains {} rows", num_rows);

    Ok(())
}
