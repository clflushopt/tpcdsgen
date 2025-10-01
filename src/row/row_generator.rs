use crate::config::Session;
use crate::row::TableRow;

/// Result of row generation (RowGeneratorResult)
pub struct RowGeneratorResult {
    rows: Vec<Box<dyn TableRow>>,
    should_end_row: bool,
}

impl RowGeneratorResult {
    /// Create a result with a single row (constructor)
    pub fn new(row: Box<dyn TableRow>) -> Self {
        Self {
            rows: vec![row],
            should_end_row: true,
        }
    }

    /// Create a result with multiple rows and end flag
    pub fn new_with_multiple(rows: Vec<Box<dyn TableRow>>, should_end_row: bool) -> Self {
        Self {
            rows,
            should_end_row,
        }
    }

    /// Get the generated rows
    pub fn get_rows(&self) -> &[Box<dyn TableRow>] {
        &self.rows
    }

    /// Check if row generation should end
    pub fn should_end_row(&self) -> bool {
        self.should_end_row
    }
}

/// RowGenerator trait matching the Java RowGenerator interface
pub trait RowGenerator: Send + Sync {
    /// Generate a row and its child rows (generateRowAndChildRows)
    fn generate_row_and_child_rows(
        &mut self,
        row_number: i64,
        session: &Session,
        parent_row_generator: Option<&mut dyn RowGenerator>,
        child_row_generator: Option<&mut dyn RowGenerator>,
    ) -> crate::error::Result<RowGeneratorResult>;

    /// Consume remaining seeds for the current row
    fn consume_remaining_seeds_for_row(&mut self);

    /// Skip rows until reaching the starting row number
    fn skip_rows_until_starting_row_number(&mut self, starting_row_number: i64);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row::call_center_row::CallCenterRow;

    #[test]
    fn test_row_generator_result_single() {
        let row = Box::new(CallCenterRow::builder().build());
        let result = RowGeneratorResult::new(row);

        assert_eq!(result.get_rows().len(), 1);
        assert!(result.should_end_row());
    }

    #[test]
    fn test_row_generator_result_multiple() {
        let rows = vec![
            Box::new(CallCenterRow::builder().build()) as Box<dyn TableRow>,
            Box::new(CallCenterRow::builder().build()) as Box<dyn TableRow>,
        ];
        let result = RowGeneratorResult::new_with_multiple(rows, false);

        assert_eq!(result.get_rows().len(), 2);
        assert!(!result.should_end_row());
    }
}
