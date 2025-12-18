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

//! Binary to generate inventory table data

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::output::Iso8859Writer;
use tpcdsgen::row::{InventoryRowGenerator, RowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = InventoryRowGenerator::new();

    let inventory_path = Path::new("inventory.dat");
    let inventory_file = File::create(inventory_path)?;
    let mut inventory_writer = Iso8859Writer::new(BufWriter::new(inventory_file));

    // Get row count for inventory at scale 1
    // Inventory row count = item_id_count × warehouse_count × weeks
    let scaling = session.get_scaling();
    let item_count = scaling.get_id_count(tpcdsgen::config::Table::Item);
    let warehouse_count = scaling.get_row_count(tpcdsgen::config::Table::Warehouse);

    // Calculate number of weeks in the date range
    let n_days =
        tpcdsgen::types::Date::JULIAN_DATE_MAXIMUM - tpcdsgen::types::Date::JULIAN_DATE_MINIMUM;
    let n_weeks = (n_days + 7) / 7; // Round up

    let num_rows = item_count * warehouse_count * n_weeks as i64;

    println!(
        "Generating inventory: {} items × {} warehouses × {} weeks = {} rows...",
        item_count, warehouse_count, n_weeks, num_rows
    );

    let mut row_count = 0i64;

    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;

        let rows = result.get_rows();
        if !rows.is_empty() {
            let values = rows[0].get_values();
            let csv_line = format!("{}|", values.join("|"));
            inventory_writer.write_line(&csv_line)?;
            row_count += 1;

            if row_count <= 3 {
                println!("Inventory Row {}: {}", row_count, csv_line);
            }
        }

        generator.consume_remaining_seeds_for_row();

        // Progress update every 100k rows
        if row_number % 100000 == 0 {
            println!(
                "Progress: {} / {} rows ({}%)",
                row_number,
                num_rows,
                row_number * 100 / num_rows
            );
        }
    }

    inventory_writer.flush()?;

    println!(
        "Generated {} inventory rows written to: {}",
        row_count,
        inventory_path.display()
    );

    Ok(())
}
