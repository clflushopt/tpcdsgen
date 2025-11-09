use crate::generator::GeneratorColumn;
use crate::random::RandomNumberStream;
use crate::table::Table;
use std::collections::HashMap;

/// Abstract base for row generators (AbstractRowGenerator)
/// Handles common functionality like random number stream management
pub struct AbstractRowGenerator {
    table: Table,
    random_number_streams: HashMap<i32, Box<dyn RandomNumberStream>>,
}

impl AbstractRowGenerator {
    /// Create a new abstract row generator for the given table
    pub fn new(table: Table) -> Self {
        Self {
            table,
            random_number_streams: HashMap::new(),
        }
    }

    /// Get the table this generator is for
    pub fn get_table(&self) -> Table {
        self.table
    }

    /// Get or create a random number stream for a generator column
    pub fn get_random_number_stream(
        &mut self,
        column: &dyn GeneratorColumn,
    ) -> &mut dyn RandomNumberStream {
        let global_column_number = column.get_global_column_number();

        self
            .random_number_streams.entry(global_column_number).or_insert_with(|| {
            // Create a new stream for this column
            let seeds_per_row = column.get_seeds_per_row();
            let stream = crate::random::RandomNumberStreamImpl::new_with_column(
                global_column_number,
                seeds_per_row,
            )
            .expect("Failed to create random number stream");
            Box::new(stream)
        });

        self.random_number_streams
            .get_mut(&global_column_number)
            .unwrap()
            .as_mut()
    }

    /// Consume remaining seeds for all streams (AbstractRowGenerator.consumeRemainingSeedsForRow)
    pub fn consume_remaining_seeds_for_row(&mut self) {
        use crate::random::RandomValueGenerator;

        for stream in self.random_number_streams.values_mut() {
            // Consume remaining seeds until each stream has used its full seeds_per_row allocation
            while stream.get_seeds_used() < stream.get_seeds_per_row() {
                RandomValueGenerator::generate_uniform_random_int(1, 100, stream.as_mut());
            }
            // Reset seeds used count for next row
            stream.reset_seeds_used();
        }
    }

    /// Skip rows for all streams until reaching the starting row number
    pub fn skip_rows_until_starting_row_number(&mut self, starting_row_number: i64) {
        for stream in self.random_number_streams.values_mut() {
            stream.skip_rows(starting_row_number);
        }
    }

    /// Advance all streams to the next row
    pub fn advance_to_next_row(&mut self, _row_number: i64) {
        // Get generator columns for this table
        let generator_column_count = self.table.get_generator_column_count();

        for i in 0..generator_column_count {
            if let Some(gen_col) = self.table.get_generator_column_by_index(i) {
                let global_column_number = gen_col.get_global_column_number();
                let seeds_per_row = gen_col.get_seeds_per_row();

                if let Some(stream) = self.random_number_streams.get_mut(&global_column_number) {
                    // Advance the stream by the number of seeds this column uses per row
                    for _ in 0..seeds_per_row {
                        stream.next_random();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::CallCenterGeneratorColumn;

    #[test]
    fn test_abstract_row_generator_creation() {
        let generator = AbstractRowGenerator::new(Table::CallCenter);
        assert_eq!(generator.get_table(), Table::CallCenter);
    }

    #[test]
    fn test_random_number_stream_creation() {
        let mut generator = AbstractRowGenerator::new(Table::CallCenter);
        let column = &CallCenterGeneratorColumn::CcCallCenterSk;

        let _stream1 = generator.get_random_number_stream(column);
        let _stream2 = generator.get_random_number_stream(column);

        // Should reuse the same stream for the same column
        assert_eq!(generator.random_number_streams.len(), 1);
    }

    #[test]
    fn test_multiple_column_streams() {
        let mut generator = AbstractRowGenerator::new(Table::CallCenter);
        let col1 = &CallCenterGeneratorColumn::CcCallCenterSk;
        let col2 = &CallCenterGeneratorColumn::CcCallCenterId;

        let _stream1 = generator.get_random_number_stream(col1);
        let _stream2 = generator.get_random_number_stream(col2);

        // Should create separate streams for different columns
        assert_eq!(generator.random_number_streams.len(), 2);
    }
}
