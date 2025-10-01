use crate::config::Table;

#[derive(Debug, Clone)]
pub struct Scaling {
    scale: f64,
}

// TODO(clflushopt): need to support the other scaling variants.
impl Scaling {
    pub fn new(scale: f64) -> Self {
        Scaling { scale }
    }

    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    /// Get row count for a table at this scale factor.
    pub fn get_row_count(&self, table: Table) -> i64 {
        let base_row_count = self.get_base_row_count(table);
        (base_row_count as f64 * self.scale) as i64
    }

    /// Get unique ID count for tables that keep history
    pub fn get_id_count(&self, table: Table) -> i64 {
        let row_count = self.get_row_count(table);
        if table.keeps_history() {
            let unique_count = (row_count / 6) * 3;
            match row_count % 6 {
                1 => unique_count + 1,
                2 | 3 => unique_count + 2,
                4 | 5 => unique_count + 3,
                _ => unique_count,
            }
        } else {
            row_count
        }
    }

    /// Basic row counts per table.
    fn get_base_row_count(&self, table: Table) -> i64 {
        match table {
            // TODO(clflushopt): Derive from scaling implementation later on.
            Table::CallCenter => 6,
            Table::CatalogPage => 11718,
            Table::CatalogReturns => 144,
            Table::CatalogSales => 1441548,
            Table::Customer => 100000,
            Table::CustomerAddress => 50000,
            Table::CustomerDemographics => 1920800,
            Table::DateDim => 73049,
            Table::HouseholdDemographics => 7200,
            Table::IncomeBand => 20,
            Table::Inventory => 11745000,
            Table::Item => 17999,
            Table::Promotion => 300,
            Table::Reason => 35,
            Table::ShipMode => 20,
            Table::Store => 12,
            Table::StoreReturns => 287514,
            Table::StoreSales => 2879987,
            Table::TimeDim => 86400,
            Table::Warehouse => 5,
            Table::WebPage => 60,
            Table::WebReturns => 71763,
            Table::WebSales => 719384,
            Table::WebSite => 30,
            Table::DbgenVersion => 1,

            Table::SBrand => 1000,
            Table::SCustomerAddress => 50000,
            Table::SCallCenter => 6,
            Table::SCatalog => 100,
            Table::SCatalogOrder => 100000,
            Table::SCatalogOrderLineitem => 500000,
            Table::SCatalogPage => 11718,
            Table::SCatalogPromotionalItem => 10000,
            Table::SCatalogReturns => 144,
            Table::SCategory => 100,
            Table::SClass => 100,
            Table::SCompany => 100,
            Table::SCustomer => 100000,
            Table::SInventory => 1000000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaling_creation() {
        let scaling = Scaling::new(1.0);
        assert_eq!(scaling.get_scale(), 1.0);
    }

    #[test]
    fn test_row_count_calculation() {
        let scaling = Scaling::new(2.0);

        // Row count should scale with scale factor
        let customer_rows = scaling.get_row_count(Table::Customer);
        assert_eq!(customer_rows, 200000); // 100000 * 2.0

        let store_rows = scaling.get_row_count(Table::Store);
        assert_eq!(store_rows, 24); // 12 * 2.0
    }

    #[test]
    fn test_id_count_for_history_tables() {
        let scaling = Scaling::new(1.0);

        // Non-history table: ID count equals row count
        let customer_ids = scaling.get_id_count(Table::Customer);
        let customer_rows = scaling.get_row_count(Table::Customer);
        assert_eq!(customer_ids, customer_rows);

        // History table: ID count is less than row count
        let item_ids = scaling.get_id_count(Table::Item);
        let item_rows = scaling.get_row_count(Table::Item);
        assert!(item_ids <= item_rows);
    }

    #[test]
    fn test_fractional_scaling() {
        let scaling = Scaling::new(0.1);
        let customer_rows = scaling.get_row_count(Table::Customer);
        assert_eq!(customer_rows, 10000); // 100000 * 0.1
    }
}
