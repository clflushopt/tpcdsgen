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

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::row::{DbgenVersionRowGenerator, RowGenerator};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = DbgenVersionRowGenerator::new();

    let output_path = Path::new("dbgen_version.dat");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Get row count for scale 1
    let num_rows = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::DbgenVersion);

    println!("Generating {} dbgen_version rows...", num_rows);

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
        "✓ Generated dbgen_version data written to: {}",
        output_path.display()
    );
    println!("✓ File contains {} rows", num_rows);

    Ok(())
}
