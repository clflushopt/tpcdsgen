pub mod call_center;
pub mod column_type;
pub mod column_types;
pub mod customer_address_column;
pub mod customer_column;
pub mod dbgen_version;
pub mod household_demographics;
pub mod inventory_column;
pub mod promotion;
pub mod web_site;

pub use call_center::CallCenterColumn;
pub use column_type::{ColumnType, ColumnTypeBase};
pub use column_types::ColumnTypes;
pub use customer_address_column::CustomerAddressColumn;
pub use customer_column::CustomerColumn;
pub use dbgen_version::DbgenVersionColumn;
pub use household_demographics::HouseholdDemographicsColumn;
pub use inventory_column::InventoryColumn;
pub use promotion::PromotionColumn;
pub use web_site::WebSiteColumn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Table {
    CallCenter,
    CatalogPage,
    CatalogReturns,
    CatalogSales,
    Warehouse,
    ShipMode,
    Reason,
    IncomeBand,
    HouseholdDemographics,
    CustomerDemographics,
    CustomerAddress,
    Customer,
    DateDim,
    TimeDim,
    Item,
    Promotion,
    Store,
    StoreReturns,
    StoreSales,
    WebPage,
    WebReturns,
    WebSales,
    WebSite,
    DbgenVersion,
    Inventory,
    // Source tables (for SCD key computation)
    SStore,
    // TODO(clflushopt): Add remaining tables
}

impl Table {
    /// Returns the name of the table in lowercase as per TPC-DS specification
    pub fn get_name(&self) -> &'static str {
        match self {
            Table::CallCenter => "call_center",
            Table::CatalogPage => "catalog_page",
            Table::CatalogReturns => "catalog_returns",
            Table::CatalogSales => "catalog_sales",
            Table::Warehouse => "warehouse",
            Table::ShipMode => "ship_mode",
            Table::Reason => "reason",
            Table::IncomeBand => "income_band",
            Table::HouseholdDemographics => "household_demographics",
            Table::CustomerDemographics => "customer_demographics",
            Table::CustomerAddress => "customer_address",
            Table::Customer => "customer",
            Table::DateDim => "date_dim",
            Table::TimeDim => "time_dim",
            Table::Item => "item",
            Table::Promotion => "promotion",
            Table::Store => "store",
            Table::StoreReturns => "store_returns",
            Table::StoreSales => "store_sales",
            Table::WebPage => "web_page",
            Table::WebReturns => "web_returns",
            Table::WebSales => "web_sales",
            Table::WebSite => "web_site",
            Table::DbgenVersion => "dbgen_version",
            Table::Inventory => "inventory",
            Table::SStore => "s_store",
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
