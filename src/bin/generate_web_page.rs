use tpcdsgen::config::Session;
use tpcdsgen::row::{RowGenerator, WebPageRowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = WebPageRowGenerator::new();

    // Get row count for scale 1 (30 rows per table metadata)
    let num_rows = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::WebPage);

    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;
        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();
        let values = rows[0].get_values();

        println!("{}|", values.join("|"));
    }

    Ok(())
}
