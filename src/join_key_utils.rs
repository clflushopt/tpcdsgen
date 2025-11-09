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

//! Join key generation utilities for TPC-DS foreign key relationships.
//!
//! This module provides functionality to generate foreign keys (join keys) between
//! TPC-DS tables, respecting the benchmark's referential integrity requirements.
//!
//! **NOTE**: Some distribution functions (CalendarDistribution, HoursDistribution) are
//! not yet fully ported. This implementation uses temporary stubs that will be replaced
//! when those distributions are complete.

use crate::config::{Scaling, Table};
use crate::error::{Result, TpcdsError};
use crate::generator::GeneratorColumn;
use crate::pseudo_table_scaling_infos::PseudoTableScalingInfos;
use crate::random::{RandomNumberStream, RandomValueGenerator};
// use crate::slowly_changing_dimension_utils;
// use crate::table::Table as MetadataTable;
use crate::types::Date;

const WEB_PAGES_PER_SITE: i32 = 123;
const WEB_DATE_STAGGER: i64 = 17;
const CS_MIN_SHIP_DELAY: i32 = 2;
const CS_MAX_SHIP_DELAY: i32 = 90;

/// Generates a join key (foreign key) from one table/column to another table.
///
/// This is the main entry point for generating foreign keys between TPC-DS tables.
/// It routes to specialized key generators based on the target table type.
///
/// # Arguments
///
/// * `from_column` - The column generating the join key
/// * `random_number_stream` - Random number stream for generation
/// * `to_table` - The target table being referenced
/// * `join_count` - Context-dependent value (often a date or row number)
/// * `scaling` - Scaling information for the dataset
///
/// # Returns
///
/// The generated join key value, or -1 if no valid key can be generated
pub fn generate_join_key(
    from_column: &dyn GeneratorColumn,
    random_number_stream: &mut dyn RandomNumberStream,
    to_table: Table,
    join_count: i64,
    scaling: &Scaling,
) -> Result<i64> {
    // NOTE: from_column.get_table() returns column::Table, not config::Table
    // For now, we pass the from_column itself and let the helper functions
    // determine the table type if needed

    match to_table {
        Table::CatalogPage => generate_catalog_page_join_key(random_number_stream, join_count, scaling),
        Table::DateDim => {
            let year = RandomValueGenerator::generate_uniform_random_int(
                Date::DATE_MINIMUM.year(),
                Date::DATE_MAXIMUM.year(),
                random_number_stream,
            );
            generate_date_join_key(
                random_number_stream,
                from_column,
                join_count,
                year,
                scaling,
            )
        }
        Table::TimeDim => generate_time_join_key(random_number_stream),
        _ => {
            if to_table.keeps_history() {
                generate_scd_join_key(to_table, random_number_stream, join_count, scaling)
            } else {
                Ok(RandomValueGenerator::generate_uniform_random_key(
                    1,
                    scaling.get_row_count(to_table),
                    random_number_stream,
                ))
            }
        }
    }
}

/// Generates a join key to the catalog_page table.
///
/// **NOTE**: This is currently stubbed as it requires CatalogPageDistributions
/// which will be ported in Phase 2.3.
fn generate_catalog_page_join_key(
    _random_number_stream: &mut dyn RandomNumberStream,
    _julian_date: i64,
    _scaling: &Scaling,
) -> Result<i64> {
    // TODO: Port CatalogPageDistributions first (Phase 2.3)
    // Then implement the full logic from JoinKeyUtils.java:generateCatalogPageJoinKey
    Err(TpcdsError::new(
        "catalog_page join keys not yet implemented - needs CatalogPageDistributions (Phase 2.3)",
    ))
}

/// Generates a join key to the date_dim table.
///
/// Different table types use different date selection strategies.
///
/// **NOTE**: This is partially stubbed as CalendarDistribution::pick_random_day_of_year
/// is not yet ported. Using uniform random for now.
///
/// Also NOTE: from_column.get_table() returns column::Table, not config::Table,
/// so we can't use table matching. For now, use column name inspection for special cases.
fn generate_date_join_key(
    random_number_stream: &mut dyn RandomNumberStream,
    _from_column: &dyn GeneratorColumn,
    _join_count: i64,
    year: i32,
    _scaling: &Scaling,
) -> Result<i64> {
    // TODO: Check for web-related columns using get_global_column_number()
    // TODO: Use CalendarDistribution when ported to detect sales vs returns vs other
    // For now, use uniform random day selection for all cases
    let max_day = if Date::is_leap_year(year) { 365 } else { 364 };
    let day_number = RandomValueGenerator::generate_uniform_random_int(0, max_day, random_number_stream);
    let result = Date::to_julian_days(&Date::new(year, 1, 1)) as i64 + day_number as i64;
    Ok(if result > Date::JULIAN_TODAYS_DATE as i64 { -1 } else { result })
}

// NOTE: This function is currently unused due to column::Table vs config::Table mismatch
// It will be used once distribution functions are ported
// /// Generates a date join key for returns tables.
// ///
// /// Returns have a lag between the sale date and return date.
// fn _generate_date_returns_join_key(
//     from_table: Table,
//     random_number_stream: &mut dyn RandomNumberStream,
//     join_count: i64,
// ) -> Result<i64> {
//     let (min, max) = match from_table {
//         Table::StoreReturns | Table::CatalogReturns => (CS_MIN_SHIP_DELAY, CS_MAX_SHIP_DELAY),
//         Table::WebReturns => (1, 120),
//         _ => {
//             return Err(TpcdsError::new(&format!(
//                 "Invalid table for date returns join: {:?}",
//                 from_table
//             )))
//         }
//     };
//
//     let lag = RandomValueGenerator::generate_uniform_random_int(min * 2, max * 2, random_number_stream);
//     Ok(join_count + lag as i64)
// }

/// Generates a join key to the time_dim table.
///
/// **NOTE**: This is partially stubbed as HoursDistribution::pick_random_hour
/// is not yet ported. Using uniform random hours for now.
fn generate_time_join_key(
    random_number_stream: &mut dyn RandomNumberStream,
) -> Result<i64> {
    // TODO: Use HoursDistribution::pick_random_hour with appropriate weights
    // For now, use uniform random hour
    let hour = RandomValueGenerator::generate_uniform_random_int(0, 23, random_number_stream);
    let seconds = RandomValueGenerator::generate_uniform_random_int(0, 3599, random_number_stream);

    Ok((hour as i64 * 3600) + seconds as i64)
}

/// Generates a join key to a slowly changing dimension (SCD) table.
///
/// SCD tables keep history, so the join key must match the appropriate
/// version of the dimension based on the effective date.
fn generate_scd_join_key(
    to_table: Table,
    random_number_stream: &mut dyn RandomNumberStream,
    julian_date: i64,
    scaling: &Scaling,
) -> Result<i64> {
    // Can't have a revision in the future
    if julian_date > Date::JULIAN_DATA_END_DATE {
        return Ok(-1);
    }

    let id_count = scaling.get_id_count(to_table);
    let mut key = RandomValueGenerator::generate_uniform_random_key(1, id_count, random_number_stream);

    // TODO: Port SlowlyChangingDimensionUtils::matchSurrogateKey from Java
    // For now, just use the key as-is without SCD matching
    // This will need to be implemented when porting SCD tables
    // let metadata_table = convert_to_metadata_table(to_table);
    // key = slowly_changing_dimension_utils::match_surrogate_key(key, julian_date, metadata_table, scaling);

    Ok(if key > scaling.get_row_count(to_table) { -1 } else { key })
}

/// Generates a join key for web-related tables (web_site, web_page).
///
/// Web tables have complex date logic involving site creation, open, and close dates.
fn generate_web_join_key(
    _from_column: &dyn GeneratorColumn,
    _random_number_stream: &mut dyn RandomNumberStream,
    _join_key: i64,
    _scaling: &Scaling,
) -> Result<i64> {
    // TODO: Port web join key generation when WebSite and WebPage tables are implemented
    // This requires identifying specific columns by get_global_column_number()
    // For now, return error as web tables aren't ported yet
    Err(TpcdsError::new(
        "Web join keys not yet implemented - needed when porting web_site and web_page tables",
    ))
}

// TODO: Uncomment when web join keys are implemented
// /// Calculates the duration of a web site based on concurrent sites.
// fn get_web_site_duration(scaling: &Scaling) -> i64 {
//     let concurrent_web_sites = PseudoTableScalingInfos::get_concurrent_web_sites();
//     let row_count = concurrent_web_sites
//         .get_row_count_for_scale(scaling.get_scale())
//         .expect("Failed to get row count for concurrent web sites");
//
//     (Date::JULIAN_DATE_MAXIMUM as i64 - Date::JULIAN_DATE_MINIMUM as i64) * row_count
// }
//
// /// Checks if a web site is replaced (has an even join key).
// fn is_replaced(join_key: i64) -> bool {
//     (join_key % 2) == 0
// }
//
// /// Checks if a web site is a replacement (odd division by 2).
// fn is_replacement(join_key: i64) -> bool {
//     (join_key / 2 % 2) != 0
// }

// TODO: Uncomment when SlowlyChangingDimensionUtils::match_surrogate_key is ported
// /// Helper function to convert config::Table to table::Table for SCD utilities.
// fn convert_to_metadata_table(table: Table) -> MetadataTable {
//     match table {
//         Table::CallCenter => MetadataTable::CallCenter,
//         Table::Warehouse => MetadataTable::Warehouse,
//         Table::ShipMode => MetadataTable::ShipMode,
//         Table::Reason => MetadataTable::Reason,
//         Table::IncomeBand => MetadataTable::IncomeBand,
//         Table::CustomerDemographics => MetadataTable::CustomerDemographics,
//         Table::DateDim => MetadataTable::DateDim,
//         Table::TimeDim => MetadataTable::TimeDim,
//         _ => panic!("Table {:?} not yet implemented in metadata table enum", table),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::RandomNumberStreamImpl;

    // TODO: Uncomment when web functions are implemented
    // #[test]
    // fn test_is_replaced() {
    //     assert!(is_replaced(0));
    //     assert!(is_replaced(2));
    //     assert!(is_replaced(4));
    //     assert!(!is_replaced(1));
    //     assert!(!is_replaced(3));
    // }
    //
    // #[test]
    // fn test_is_replacement() {
    //     assert!(!is_replacement(0)); // 0/2=0, 0%2=0
    //     assert!(!is_replacement(1)); // 1/2=0, 0%2=0
    //     assert!(is_replacement(2)); // 2/2=1, 1%2=1
    //     assert!(is_replacement(3)); // 3/2=1, 1%2=1
    //     assert!(!is_replacement(4)); // 4/2=2, 2%2=0
    // }

    #[test]
    fn test_generate_time_join_key() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let result = generate_time_join_key(&mut stream).unwrap();

        // Time keys should be in range [0, 86400) seconds in a day
        assert!(result >= 0 && result < 86400, "Time key should be valid seconds in day");
    }

    #[test]
    fn test_generate_time_join_key_deterministic() {
        let mut stream1 = RandomNumberStreamImpl::new(1).unwrap();
        let mut stream2 = RandomNumberStreamImpl::new(1).unwrap();

        let result1 = generate_time_join_key(&mut stream1).unwrap();
        let result2 = generate_time_join_key(&mut stream2).unwrap();

        assert_eq!(result1, result2, "Same seed should produce same time key");
    }

    #[test]
    fn test_catalog_page_join_key_is_stubbed() {
        let mut stream = RandomNumberStreamImpl::new(1).unwrap();
        let scaling = Scaling::new(1.0);

        let result = generate_catalog_page_join_key(&mut stream, 2451545, &scaling);
        assert!(result.is_err(), "Catalog page join should be stubbed");

        if let Err(e) = result {
            assert!(
                e.message().contains("CatalogPageDistributions"),
                "Error should mention missing distribution"
            );
        }
    }

    // NOTE: Test disabled until column::Table vs config::Table is resolved
    // #[test]
    // fn test_generate_date_returns_join_key() {
    //     let mut stream = RandomNumberStreamImpl::new(1).unwrap();
    //     let sale_date = Date::to_julian_days(&Date::new(2003, 1, 1)) as i64;
    //
    //     let return_date = _generate_date_returns_join_key(
    //         Table::StoreReturns,
    //         &mut stream,
    //         sale_date,
    //     )
    //     .unwrap();
    //
    //     // Return should be after sale
    //     assert!(return_date > sale_date, "Return date should be after sale date");
    //
    //     // Lag should be within expected range
    //     let lag = return_date - sale_date;
    //     assert!(lag >= (CS_MIN_SHIP_DELAY * 2) as i64 && lag <= (CS_MAX_SHIP_DELAY * 2) as i64);
    // }
}
