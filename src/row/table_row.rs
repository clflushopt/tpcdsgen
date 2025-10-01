/// TableRow trait matching the Java TableRow interface
/// Represents a single row of data from any TPC-DS table
pub trait TableRow: Send + Sync {
    /// Get all values as strings for output (getValues())
    fn get_values(&self) -> Vec<String>;
    
    /// Get the number of columns in this row
    fn get_column_count(&self) -> usize {
        self.get_values().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a simple test implementation
    struct TestTableRow {
        values: Vec<String>,
    }

    impl TableRow for TestTableRow {
        fn get_values(&self) -> Vec<String> {
            self.values.clone()
        }
    }

    #[test]
    fn test_table_row_trait() {
        let test_row = TestTableRow {
            values: vec!["1".to_string(), "test".to_string(), "123.45".to_string()],
        };

        let values = test_row.get_values();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], "1");
        assert_eq!(values[1], "test");
        assert_eq!(values[2], "123.45");
        assert_eq!(test_row.get_column_count(), 3);
    }
}