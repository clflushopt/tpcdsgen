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

//! Binary to generate store table data

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tpcdsgen::config::Session;
use tpcdsgen::output::Iso8859Writer;
use tpcdsgen::row::{RowGenerator, StoreRowGenerator, TableRow};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let session = Session::get_default_session();
    let mut generator = StoreRowGenerator::new();

    let output_path = Path::new("store.dat");
    let file = File::create(output_path)?;
    let mut writer = Iso8859Writer::new(BufWriter::new(file));

    // Get row count for scale 1
    let num_rows = session
        .get_scaling()
        .get_row_count(tpcdsgen::config::Table::Store);

    println!("Generating {} store rows...", num_rows);

    for row_number in 1..=num_rows {
        let result = generator.generate_row_and_child_rows(row_number, &session, None, None)?;
        generator.consume_remaining_seeds_for_row();

        let rows = result.get_rows();

        for row in rows {
            // Use streaming write_to instead of allocating Vec<String>
            row.write_to(&mut writer, '|')?;

            if row_number <= 3 {
                let values = row.get_values();
                println!("Row {}: {}|", row_number, values.join("|"));
            }
        }
    }

    writer.flush()?;
    println!("Generated store data written to: {}", output_path.display());
    println!("File contains {} rows", num_rows);

    Ok(())
}
