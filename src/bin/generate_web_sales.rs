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

//! Binary to generate web_sales and web_returns table data

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::output::Iso8859Writer;
use tpcdsgen::row::{RowGenerator, WebSalesRowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = WebSalesRowGenerator::new();

    let web_sales_path = Path::new("web_sales.dat");
    let web_returns_path = Path::new("web_returns.dat");

    let web_sales_file = File::create(web_sales_path)?;
    let web_returns_file = File::create(web_returns_path)?;

    let mut web_sales_writer = Iso8859Writer::new(BufWriter::new(web_sales_file));
    let mut web_returns_writer = Iso8859Writer::new(BufWriter::new(web_returns_file));

    // Get row count (number of ORDERS) for scale 1
    let num_orders = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::WebSales);

    println!("Generating web_sales from {} orders...", num_orders);

    let mut web_sales_count = 0;
    let mut web_returns_count = 0;
    let mut row_number = 1i64;

    // Iterate by ORDER number like Java does
    // row_number only increments when shouldEndRow() is true
    while row_number <= num_orders {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        let rows = result.get_rows();

        // First row is always web_sales
        if !rows.is_empty() {
            // Use streaming write_to instead of allocating Vec<String>
            rows[0].write_to(&mut web_sales_writer, '|')?;
            web_sales_count += 1;

            if web_sales_count <= 3 {
                let values = rows[0].get_values();
                println!("Web Sales Row {}: {}|", web_sales_count, values.join("|"));
            }
        }

        // Second row (if present) is web_returns
        if rows.len() > 1 {
            rows[1].write_to(&mut web_returns_writer, '|')?;
            web_returns_count += 1;

            if web_returns_count <= 3 {
                let values = rows[1].get_values();
                println!(
                    "Web Returns Row {}: {}|",
                    web_returns_count,
                    values.join("|")
                );
            }
        }

        // Only consume seeds and increment row_number at end of order (like Java's Results.rowStop())
        if result.should_end_row() {
            generator.consume_remaining_seeds_for_row();
            generator.consume_child_seeds(); // Also consume child (web_returns) generator seeds
            row_number += 1;
        }
    }

    web_sales_writer.flush()?;
    web_returns_writer.flush()?;

    println!(
        "Generated {} web_sales rows written to: {}",
        web_sales_count,
        web_sales_path.display()
    );
    println!(
        "Generated {} web_returns rows written to: {}",
        web_returns_count,
        web_returns_path.display()
    );

    Ok(())
}
