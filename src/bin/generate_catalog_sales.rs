/*
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Binary to generate catalog_sales and catalog_returns table data

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::output::Iso8859Writer;
use tpcdsgen::row::{CatalogSalesRowGenerator, RowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = CatalogSalesRowGenerator::new();

    let catalog_sales_path = Path::new("catalog_sales.dat");
    let catalog_returns_path = Path::new("catalog_returns.dat");

    let catalog_sales_file = File::create(catalog_sales_path)?;
    let catalog_returns_file = File::create(catalog_returns_path)?;

    let mut catalog_sales_writer = Iso8859Writer::new(BufWriter::new(catalog_sales_file));
    let mut catalog_returns_writer = Iso8859Writer::new(BufWriter::new(catalog_returns_file));

    // Get row count (number of ORDERS) for scale 1
    let num_orders = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::CatalogSales);

    println!("Generating catalog_sales from {} orders...", num_orders);

    let mut catalog_sales_count = 0;
    let mut catalog_returns_count = 0;
    let mut row_number = 1i64;

    // Iterate by ORDER number like Java does
    // row_number only increments when shouldEndRow() is true
    while row_number <= num_orders {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        let rows = result.get_rows();

        // First row is always catalog_sales
        if !rows.is_empty() {
            let values = rows[0].get_values();
            let csv_line = format!("{}|", values.join("|"));
            catalog_sales_writer.write_line(&csv_line)?;
            catalog_sales_count += 1;

            if catalog_sales_count <= 3 {
                println!("Catalog Sales Row {}: {}", catalog_sales_count, csv_line);
            }
        }

        // Second row (if present) is catalog_returns
        if rows.len() > 1 {
            let values = rows[1].get_values();
            let csv_line = format!("{}|", values.join("|"));
            catalog_returns_writer.write_line(&csv_line)?;
            catalog_returns_count += 1;

            if catalog_returns_count <= 3 {
                println!(
                    "Catalog Returns Row {}: {}",
                    catalog_returns_count, csv_line
                );
            }
        }

        // Only consume seeds and increment row_number at end of order (like Java's Results.rowStop())
        if result.should_end_row() {
            generator.consume_remaining_seeds_for_row();
            generator.consume_child_seeds(); // Also consume child (catalog_returns) generator seeds
            row_number += 1;
        }
    }

    catalog_sales_writer.flush()?;
    catalog_returns_writer.flush()?;

    println!(
        "Generated {} catalog_sales rows written to: {}",
        catalog_sales_count,
        catalog_sales_path.display()
    );
    println!(
        "Generated {} catalog_returns rows written to: {}",
        catalog_returns_count,
        catalog_returns_path.display()
    );

    Ok(())
}
