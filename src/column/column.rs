use crate::column::ColumnType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Table {
    CallCenter,
    Warehouse,
    ShipMode,
    Reason,
    IncomeBand,
    HouseholdDemographics,
    CustomerDemographics,
    DateDim,
    TimeDim,
    // TODO(clflushopt): Add remaining tables
}

impl Table {
    /// Returns the name of the table in lowercase as per TPC-DS specification
    pub fn get_name(&self) -> &'static str {
        match self {
            Table::CallCenter => "call_center",
            Table::Warehouse => "warehouse",
            Table::ShipMode => "ship_mode",
            Table::Reason => "reason",
            Table::IncomeBand => "income_band",
            Table::HouseholdDemographics => "household_demographics",
            Table::CustomerDemographics => "customer_demographics",
            Table::DateDim => "date_dim",
            Table::TimeDim => "time_dim",
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

/// TODO(clflushopt): We probably don't need this but Java keeps it around.
pub trait Column: Send + Sync {
    /// Get the table this column belongs to
    fn get_table(&self) -> Table;

    /// Get the column name (lowercase)
    fn get_name(&self) -> &'static str;

    /// Get the column type
    fn get_type(&self) -> &ColumnType;

    /// Get the column position (0-based ordinal)
    fn get_position(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_name() {
        assert_eq!(Table::CallCenter.get_name(), "call_center");
        assert_eq!(format!("{}", Table::CallCenter), "call_center");
    }
}
