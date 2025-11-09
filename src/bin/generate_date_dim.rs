use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::{Session, Table};
use tpcdsgen::row::{DateDimRowGenerator, RowGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = DateDimRowGenerator::new();

    let output_path = Path::new("date_dim.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Get number of rows for date_dim at scale 1
    let num_rows = session.get_scaling().get_row_count(Table::DateDim);

    println!("Generating {} date_dim rows...", num_rows);

    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        // Consume any remaining seeds
        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();
        for row in rows {
            let values = row.get_values();
            let csv_line = values.join("|");
            writeln!(writer, "{}|", csv_line)?;
        }

        if row_number % 10000 == 0 {
            println!("Progress: {} rows generated", row_number);
        }
    }

    writer.flush()?;
    println!(
        "Generated date_dim data written to: {}",
        output_path.display()
    );
    println!("File contains {} rows", num_rows);

    Ok(())
}
