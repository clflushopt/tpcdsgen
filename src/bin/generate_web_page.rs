use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{RowGenerator, WebPageRowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = WebPageRowGenerator::new();

    let output_path = Path::new("web_page.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Get row count for scale 1
    let num_rows = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::WebPage);

    println!("Generating {} web_page rows...", num_rows);

    for row_number in 1..=num_rows {
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
        "✓ Generated web_page data written to: {}",
        output_path.display()
    );
    println!("✓ File contains {} rows", num_rows);

    Ok(())
}
