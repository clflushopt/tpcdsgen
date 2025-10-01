use crate::business_key_generator::make_business_key;
use crate::table::Table;
use crate::types::Date;

const ONE_HALF_DATE: i64 =
    Date::JULIAN_DATA_START_DATE + (Date::JULIAN_DATA_END_DATE - Date::JULIAN_DATA_START_DATE) / 2;
const ONE_THIRD_PERIOD: i64 = (Date::JULIAN_DATA_END_DATE - Date::JULIAN_DATA_START_DATE) / 3;
const ONE_THIRD_DATE: i64 = Date::JULIAN_DATA_START_DATE + ONE_THIRD_PERIOD;
const TWO_THIRDS_DATE: i64 = ONE_THIRD_DATE + ONE_THIRD_PERIOD;

#[derive(Debug, Clone)]
pub struct SlowlyChangingDimensionKey {
    business_key: String,
    start_date: i64,
    end_date: i64,
    is_new_business_key: bool,
}

impl SlowlyChangingDimensionKey {
    pub fn new(
        business_key: String,
        start_date: i64,
        end_date: i64,
        is_new_business_key: bool,
    ) -> Self {
        Self {
            business_key,
            start_date,
            end_date,
            is_new_business_key,
        }
    }

    pub fn get_business_key(&self) -> &str {
        &self.business_key
    }

    pub fn get_start_date(&self) -> i64 {
        self.start_date
    }

    pub fn get_end_date(&self) -> i64 {
        self.end_date
    }

    pub fn is_new_business_key(&self) -> bool {
        self.is_new_business_key
    }
}

pub fn compute_scd_key(table: Table, row_number: i64) -> SlowlyChangingDimensionKey {
    let modulo = (row_number % 6) as i32;
    let table_number = table as i64;

    let (business_key, start_date, mut end_date, is_new_key) = match modulo {
        1 => {
            // 1 revision
            let business_key = make_business_key(row_number);
            let start_date = Date::JULIAN_DATA_START_DATE - table_number * 6;
            let end_date = -1;
            (business_key, start_date, end_date, true)
        }
        2 => {
            // 1 of 2 revisions
            let business_key = make_business_key(row_number);
            let start_date = Date::JULIAN_DATA_START_DATE - table_number * 6;
            let end_date = ONE_HALF_DATE - table_number * 6;
            (business_key, start_date, end_date, true)
        }
        3 => {
            // 2 of 2 revisions
            let business_key = make_business_key(row_number - 1);
            let start_date = ONE_HALF_DATE - table_number * 6 + 1;
            let end_date = -1;
            (business_key, start_date, end_date, false)
        }
        4 => {
            // 1 of 3 revisions
            let business_key = make_business_key(row_number);
            let start_date = Date::JULIAN_DATA_START_DATE - table_number * 6;
            let end_date = ONE_THIRD_DATE - table_number * 6;
            (business_key, start_date, end_date, true)
        }
        5 => {
            // 2 of 3 revisions
            let business_key = make_business_key(row_number - 1);
            let start_date = ONE_THIRD_DATE - table_number * 6 + 1;
            let end_date = TWO_THIRDS_DATE - table_number * 6;
            (business_key, start_date, end_date, false)
        }
        0 => {
            // 3 of 3 revisions
            let business_key = make_business_key(row_number - 2);
            let start_date = TWO_THIRDS_DATE - table_number * 6 + 1;
            let end_date = -1;
            (business_key, start_date, end_date, false)
        }
        _ => panic!(
            "Something's wrong. Positive integers % 6 should always be covered by one of the cases"
        ),
    };

    if end_date > Date::JULIAN_DATA_END_DATE {
        end_date = -1;
    }

    SlowlyChangingDimensionKey::new(business_key, start_date, end_date, is_new_key)
}

pub fn get_value_for_slowly_changing_dimension<T>(
    field_change_flag: i32,
    is_new_key: bool,
    old_value: T,
    new_value: T,
) -> T {
    if should_change_dimension(field_change_flag, is_new_key) {
        new_value
    } else {
        old_value
    }
}

pub fn should_change_dimension(flags: i32, is_new_key: bool) -> bool {
    flags % 2 == 0 || is_new_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_change_dimension() {
        assert!(should_change_dimension(0, false)); // even flag
        assert!(!should_change_dimension(1, false)); // odd flag
        assert!(should_change_dimension(1, true)); // new key overrides odd flag
        assert!(should_change_dimension(0, true)); // new key + even flag
    }
}
