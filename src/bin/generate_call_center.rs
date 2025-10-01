use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{CallCenterRowGenerator, RowGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();

    let mut generator = CallCenterRowGenerator::new();

    let output_path = Path::new("call_center.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let num_call_centers = 6;

    println!("Generating {} call center rows...", num_call_centers);

    for row_number in 1..=num_call_centers {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();

        for row in rows {
            let values = row.get_values();

            let csv_line = values.join("|");
            writeln!(writer, "{}|", csv_line)?;

            if row_number <= 3 {
                println!("Row {}: {}", row_number, csv_line);
            }
        }
    }

    writer.flush()?;
    println!(
        "✓ Generated call center data written to: {}",
        output_path.display()
    );
    println!("✓ File contains {} rows", num_call_centers);

    Ok(())
}
