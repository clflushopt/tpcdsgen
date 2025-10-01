use crate::column::{CallCenterColumn, Column};
use crate::error::Result;
use crate::generator::{
    CallCenterGeneratorColumn, CustomerDemographicsGeneratorColumn, GeneratorColumn,
    IncomeBandGeneratorColumn, ReasonGeneratorColumn, ShipModeGeneratorColumn,
    WarehouseGeneratorColumn,
};
use crate::scaling_info::{ScalingInfo, ScalingModel};
use crate::table_flags::{TableFlags, TableFlagsBuilder};
use std::sync::OnceLock;

/// Table enum representing all TPC-DS tables with complete metadata (Table)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Table {
    CallCenter,
    Warehouse,
    ShipMode,
    Reason,
    IncomeBand,
    CustomerDemographics,
    // TODO: Add other tables as they are implemented
}

impl Table {
    /// Get the lowercase table name (getName())
    pub fn get_name(&self) -> &'static str {
        match self {
            Table::CallCenter => "call_center",
            Table::Warehouse => "warehouse",
            Table::ShipMode => "ship_mode",
            Table::Reason => "reason",
            Table::IncomeBand => "income_band",
            Table::CustomerDemographics => "customer_demographics",
        }
    }

    /// Get table flags
    pub fn get_table_flags(&self) -> &'static TableFlags {
        match self {
            Table::CallCenter => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| {
                    TableFlagsBuilder::new()
                        .set_is_small()
                        .set_keeps_history()
                        .build()
                })
            }
            Table::Warehouse => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| TableFlagsBuilder::new().set_is_small().build())
            }
            Table::ShipMode => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| TableFlagsBuilder::new().set_is_small().build())
            }
            Table::Reason => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| TableFlagsBuilder::new().set_is_small().build())
            }
            Table::IncomeBand => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| TableFlagsBuilder::new().set_is_small().build())
            }
            Table::CustomerDemographics => {
                static FLAGS: OnceLock<TableFlags> = OnceLock::new();
                FLAGS.get_or_init(|| TableFlagsBuilder::new().build())
            }
        }
    }

    /// Get null basis points for this table
    pub fn get_null_basis_points(&self) -> i32 {
        match self {
            Table::CallCenter => 100,
            Table::Warehouse => 100,
            Table::ShipMode => 100,
            Table::Reason => 100,
            Table::IncomeBand => 0,
            Table::CustomerDemographics => 0,
        }
    }

    /// Get not-null bitmap for this table
    pub fn get_not_null_bit_map(&self) -> i64 {
        match self {
            Table::CallCenter => 0xB,
            Table::Warehouse => 0x3,
            Table::ShipMode => 0x3,
            Table::Reason => 0x3,
            Table::IncomeBand => 0x1,
            Table::CustomerDemographics => 0x1,
        }
    }

    /// Get scaling info for this table
    pub fn get_scaling_info(&self) -> &'static ScalingInfo {
        match self {
            Table::CallCenter => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [0, 3, 12, 15, 18, 21, 24, 27, 30, 30];
                    ScalingInfo::new(0, ScalingModel::Logarithmic, &row_counts, 0)
                        .expect("CallCenter ScalingInfo creation should not fail")
                })
            }
            Table::Warehouse => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [0, 1, 1, 1, 1, 1, 1, 1, 1, 5];
                    ScalingInfo::new(0, ScalingModel::Static, &row_counts, 0)
                        .expect("Warehouse ScalingInfo creation should not fail")
                })
            }
            Table::ShipMode => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [0, 1, 1, 1, 1, 1, 1, 1, 1, 20];
                    ScalingInfo::new(0, ScalingModel::Static, &row_counts, 0)
                        .expect("ShipMode ScalingInfo creation should not fail")
                })
            }
            Table::Reason => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [0, 1, 1, 1, 1, 1, 1, 1, 1, 35];
                    ScalingInfo::new(0, ScalingModel::Static, &row_counts, 0)
                        .expect("Reason ScalingInfo creation should not fail")
                })
            }
            Table::IncomeBand => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [0, 1, 1, 1, 1, 1, 1, 1, 1, 20];
                    ScalingInfo::new(0, ScalingModel::Static, &row_counts, 0)
                        .expect("IncomeBand ScalingInfo creation should not fail")
                })
            }
            Table::CustomerDemographics => {
                static SCALING: OnceLock<ScalingInfo> = OnceLock::new();
                SCALING.get_or_init(|| {
                    let row_counts = [
                        0, 19208, 19208, 19208, 19208, 19208, 19208, 19208, 19208, 19208,
                    ];
                    ScalingInfo::new(2, ScalingModel::Static, &row_counts, 0)
                        .expect("CustomerDemographics ScalingInfo creation should not fail")
                })
            }
        }
    }

    /// Get regular column count for this table
    pub fn get_column_count(&self) -> usize {
        match self {
            Table::CallCenter => CallCenterColumn::values().len(),
            Table::Warehouse => 0, // TODO: Return WarehouseColumn::values().len() once WarehouseColumn is implemented
            Table::ShipMode => 0, // TODO: Return ShipModeColumn::values().len() once ShipModeColumn is implemented
            Table::Reason => 0, // TODO: Return ReasonColumn::values().len() once ReasonColumn is implemented
            Table::IncomeBand => 0, // TODO: Return IncomeBandColumn::values().len() once IncomeBandColumn is implemented
            Table::CustomerDemographics => 0, // TODO: Return CustomerDemographicsColumn::values().len() once CustomerDemographicsColumn is implemented
        }
    }

    /// Get generator column count for this table
    pub fn get_generator_column_count(&self) -> usize {
        match self {
            Table::CallCenter => CallCenterGeneratorColumn::values().len(),
            Table::Warehouse => WarehouseGeneratorColumn::values().len(),
            Table::ShipMode => ShipModeGeneratorColumn::values().len(),
            Table::Reason => ReasonGeneratorColumn::values().len(),
            Table::IncomeBand => IncomeBandGeneratorColumn::values().len(),
            Table::CustomerDemographics => CustomerDemographicsGeneratorColumn::values().len(),
        }
    }

    /// Get a specific regular column by index
    pub fn get_column_by_index(&self, index: usize) -> Option<&'static dyn Column> {
        match self {
            Table::CallCenter => {
                let columns = CallCenterColumn::values();
                columns.get(index).map(|col| col as &dyn Column)
            }
            Table::Warehouse => {
                // TODO: Implement once WarehouseColumn is created
                None
            }
            Table::ShipMode => {
                // TODO: Implement once ShipModeColumn is created
                None
            }
            Table::Reason => {
                // TODO: Implement once ReasonColumn is created
                None
            }
            Table::IncomeBand => {
                // TODO: Implement once IncomeBandColumn is created
                None
            }
            Table::CustomerDemographics => {
                // TODO: Implement once CustomerDemographicsColumn is created
                None
            }
        }
    }

    /// Get a specific generator column by index
    pub fn get_generator_column_by_index(
        &self,
        index: usize,
    ) -> Option<&'static dyn GeneratorColumn> {
        match self {
            Table::CallCenter => {
                let columns = CallCenterGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
            Table::Warehouse => {
                let columns = WarehouseGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
            Table::ShipMode => {
                let columns = ShipModeGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
            Table::Reason => {
                let columns = ReasonGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
            Table::IncomeBand => {
                let columns = IncomeBandGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
            Table::CustomerDemographics => {
                let columns = CustomerDemographicsGeneratorColumn::values();
                columns.get(index).map(|col| col as &dyn GeneratorColumn)
            }
        }
    }

    /// Get a specific column by name (case-insensitive)
    pub fn get_column(&self, column_name: &str) -> Result<&'static dyn Column> {
        let column_name_lower = column_name.to_lowercase();
        let column_count = self.get_column_count();

        let mut found_column = None;
        for i in 0..column_count {
            if let Some(col) = self.get_column_by_index(i) {
                if col.get_name().to_lowercase() == column_name_lower {
                    if found_column.is_some() {
                        return Err(crate::TpcdsError::new(&format!(
                            "Multiple columns found matching '{}' in table '{}'",
                            column_name,
                            self.get_name()
                        )));
                    }
                    found_column = Some(col);
                }
            }
        }

        found_column.ok_or_else(|| {
            crate::TpcdsError::new(&format!(
                "Column '{}' not found in table '{}'",
                column_name,
                self.get_name()
            ))
        })
    }

    /// Check if this table keeps history
    pub fn keeps_history(&self) -> bool {
        self.get_table_flags().keeps_history()
    }

    /// Check if this is a small table
    pub fn is_small(&self) -> bool {
        self.get_table_flags().is_small()
    }

    /// Check if this table is date-based
    pub fn is_date_based(&self) -> bool {
        self.get_table_flags().is_date_based()
    }

    /// Get all base tables (non-source tables)
    pub fn get_base_tables() -> Vec<Table> {
        vec![
            Table::CallCenter,
            Table::Warehouse,
            Table::ShipMode,
            Table::Reason,
            Table::IncomeBand,
            Table::CustomerDemographics,
        ] // TODO: Add other tables as implemented
    }

    /// Get a table by name (case-insensitive)
    pub fn get_table(table_name: &str) -> Result<Table> {
        let table_name_lower = table_name.to_lowercase();
        let base_tables = Self::get_base_tables();

        let matches: Vec<_> = base_tables
            .iter()
            .filter(|table| table.get_name() == table_name_lower)
            .collect();

        if matches.len() == 1 {
            Ok(*matches[0])
        } else if matches.is_empty() {
            Err(crate::TpcdsError::new(&format!(
                "Table '{}' not found",
                table_name
            )))
        } else {
            Err(crate::TpcdsError::new(&format!(
                "Multiple tables found matching '{}'",
                table_name
            )))
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

// Move the original simple Table from column.rs here and update column.rs to use this one
impl From<Table> for crate::column::Table {
    fn from(table: Table) -> Self {
        match table {
            Table::CallCenter => crate::column::Table::CallCenter,
            Table::Warehouse => crate::column::Table::Warehouse,
            Table::ShipMode => crate::column::Table::ShipMode,
            Table::Reason => crate::column::Table::Reason,
            Table::IncomeBand => crate::column::Table::IncomeBand,
            Table::CustomerDemographics => crate::column::Table::CustomerDemographics,
        }
    }
}

impl From<crate::column::Table> for Table {
    fn from(table: crate::column::Table) -> Self {
        match table {
            crate::column::Table::CallCenter => Table::CallCenter,
            crate::column::Table::Warehouse => Table::Warehouse,
            crate::column::Table::ShipMode => Table::ShipMode,
            crate::column::Table::Reason => Table::Reason,
            crate::column::Table::IncomeBand => Table::IncomeBand,
            crate::column::Table::CustomerDemographics => Table::CustomerDemographics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column::ColumnTypeBase;

    #[test]
    fn test_table_name() {
        assert_eq!(Table::CallCenter.get_name(), "call_center");
        assert_eq!(format!("{}", Table::CallCenter), "call_center");
    }

    #[test]
    fn test_table_flags() {
        let flags = Table::CallCenter.get_table_flags();
        assert!(flags.keeps_history());
        assert!(flags.is_small());
        assert!(!flags.is_date_based());
    }

    #[test]
    fn test_table_metadata() {
        assert_eq!(Table::CallCenter.get_null_basis_points(), 100);
        assert_eq!(Table::CallCenter.get_not_null_bit_map(), 0xB);
    }

    #[test]
    fn test_scaling_info() {
        let scaling = Table::CallCenter.get_scaling_info();
        assert_eq!(scaling.get_multiplier(), 0);
        assert_eq!(scaling.get_scaling_model(), ScalingModel::Logarithmic);

        // Test specific scale values from Java
        assert_eq!(scaling.get_row_count_for_scale(1.0).unwrap(), 3);
        assert_eq!(scaling.get_row_count_for_scale(100000.0).unwrap(), 30);
    }

    #[test]
    fn test_get_columns() {
        let table = Table::CallCenter;
        assert_eq!(table.get_column_count(), 31);

        // Test first column
        let first_col = table.get_column_by_index(0).unwrap();
        assert_eq!(first_col.get_name(), "cc_call_center_sk");
        assert_eq!(first_col.get_position(), 0);

        // Convert column table to our table type for comparison
        let column_table: Table = first_col.get_table().into();
        assert_eq!(column_table, Table::CallCenter);
    }

    #[test]
    fn test_get_generator_columns() {
        let table = Table::CallCenter;
        assert_eq!(table.get_generator_column_count(), 34);

        // Test first generator column
        let first_gen_col = table.get_generator_column_by_index(0).unwrap();
        assert_eq!(first_gen_col.get_global_column_number(), 1);

        // Convert generator column table to our table type for comparison
        let gen_column_table: Table = first_gen_col.get_table().into();
        assert_eq!(gen_column_table, Table::CallCenter);
    }

    #[test]
    fn test_get_column_by_name() {
        let table = Table::CallCenter;

        // Test exact match
        let column = table.get_column("cc_call_center_sk").unwrap();
        assert_eq!(column.get_name(), "cc_call_center_sk");

        // Test case insensitive
        let column = table.get_column("CC_CALL_CENTER_SK").unwrap();
        assert_eq!(column.get_name(), "cc_call_center_sk");

        // Test not found
        assert!(table.get_column("nonexistent_column").is_err());
    }

    #[test]
    fn test_table_flags_methods() {
        let table = Table::CallCenter;
        assert!(table.keeps_history());
        assert!(table.is_small());
        assert!(!table.is_date_based());
    }

    #[test]
    fn test_get_table_by_name() {
        // Test exact match
        let table = Table::get_table("call_center").unwrap();
        assert_eq!(table, Table::CallCenter);

        // Test case insensitive
        let table = Table::get_table("CALL_CENTER").unwrap();
        assert_eq!(table, Table::CallCenter);

        // Test not found
        assert!(Table::get_table("nonexistent_table").is_err());
    }

    #[test]
    fn test_table_conversions() {
        let table = Table::CallCenter;
        let column_table: crate::column::Table = table.into();
        let back_to_table: Table = column_table.into();
        assert_eq!(table, back_to_table);
    }

    #[test]
    fn test_column_types_integration() {
        let table = Table::CallCenter;

        // Test some specific column types by finding them by name
        let sk_column = table.get_column("cc_call_center_sk").unwrap();
        assert_eq!(sk_column.get_type().get_base(), ColumnTypeBase::Identifier);

        let name_column = table.get_column("cc_name").unwrap();
        assert_eq!(name_column.get_type().get_base(), ColumnTypeBase::Varchar);
        assert_eq!(name_column.get_type().get_precision(), Some(50));

        let date_column = table.get_column("cc_rec_start_date").unwrap();
        assert_eq!(date_column.get_type().get_base(), ColumnTypeBase::Date);
    }

    #[test]
    fn test_generator_vs_regular_column_count() {
        let table = Table::CallCenter;

        assert_eq!(table.get_column_count(), 31); // User-visible columns
        assert_eq!(table.get_generator_column_count(), 34); // Generator columns (includes address, scd, nulls)
    }

    #[test]
    fn test_singleton_behavior() {
        // Test that repeated calls return the same references
        let flags1 = Table::CallCenter.get_table_flags();
        let flags2 = Table::CallCenter.get_table_flags();
        assert!(std::ptr::eq(flags1, flags2));

        let scaling1 = Table::CallCenter.get_scaling_info();
        let scaling2 = Table::CallCenter.get_scaling_info();
        assert!(std::ptr::eq(scaling1, scaling2));
    }
}
