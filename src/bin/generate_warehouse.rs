use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{RowGenerator, WarehouseRowGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();

    let mut generator = WarehouseRowGenerator::new();

    let output_path = Path::new("warehouse.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let num_warehouses = 5;

    println!("Generating {} warehouse rows...", num_warehouses);

    for row_number in 1..=num_warehouses {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();

        for row in rows {
            let values = row.get_values();

            let csv_line = values.join("|");
            writeln!(writer, "{}|", csv_line)?;

            println!("Row {}: {}", row_number, csv_line);
        }
    }

    writer.flush()?;
    println!(
        "Generated warehouse data written to: {}",
        output_path.display()
    );
    println!("File contains {} rows", num_warehouses);

    Ok(())
}
