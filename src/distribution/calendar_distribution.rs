use crate::distribution::file_loader::DistributionFileLoader;
use crate::distribution::utils::WeightsBuilder;
use crate::error::Result;
use crate::TpcdsError;
use std::sync::OnceLock;

/// Calendar distribution for date_dim generation (CalendarDistribution)
pub struct CalendarDistribution {
    days_of_year: Vec<i32>,
    quarters: Vec<i32>,
    holiday_flags: Vec<i32>,
    weights_lists: Vec<Vec<i32>>,
}

impl CalendarDistribution {
    const NUM_WEIGHT_FIELDS: usize = 10;
    const VALUES_AND_WEIGHTS_FILENAME: &'static str = "calendar.dst";

    /// Days before each month (non-leap and leap year)
    pub const DAYS_BEFORE_MONTH: [[i32; 12]; 2] = [
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],  // Non-leap year
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],  // Leap year
    ];

    fn get_instance() -> &'static CalendarDistribution {
        static DISTRIBUTION: OnceLock<CalendarDistribution> = OnceLock::new();
        DISTRIBUTION.get_or_init(|| {
            Self::build_calendar_distribution()
                .expect("Failed to load calendar distribution")
        })
    }

    fn build_calendar_distribution() -> Result<Self> {
        let mut days_of_year = Vec::new();
        let mut quarters = Vec::new();
        let mut holiday_flags = Vec::new();
        let mut weights_builders: Vec<WeightsBuilder> = (0..Self::NUM_WEIGHT_FIELDS)
            .map(|_| WeightsBuilder::new())
            .collect();

        let parsed_lines = DistributionFileLoader::load_distribution_file(Self::VALUES_AND_WEIGHTS_FILENAME)?;

        for (values, weights) in parsed_lines {
            if values.len() != 8 {
                return Err(TpcdsError::new(&format!(
                    "Expected line to contain 8 values, but it contained {}: {:?}",
                    values.len(),
                    values
                )));
            }

            if weights.len() != Self::NUM_WEIGHT_FIELDS {
                return Err(TpcdsError::new(&format!(
                    "Expected line to contain {} weights, but it contained {}: {:?}",
                    Self::NUM_WEIGHT_FIELDS,
                    weights.len(),
                    weights
                )));
            }

            // Parse values (only use day_of_year, quarter, and holiday_flag)
            // Values are: [0]=day_of_year, [1]=month_name, [2]=day, [3]=season,
            //             [4]=month_number, [5]=quarter, [6]=first_of_month, [7]=holiday_flag
            days_of_year.push(values[0].parse().map_err(|e| {
                TpcdsError::new(&format!("Failed to parse day_of_year '{}': {}", values[0], e))
            })?);

            quarters.push(values[5].parse().map_err(|e| {
                TpcdsError::new(&format!("Failed to parse quarter '{}': {}", values[5], e))
            })?);

            holiday_flags.push(values[7].parse().map_err(|e| {
                TpcdsError::new(&format!("Failed to parse holiday_flag '{}': {}", values[7], e))
            })?);

            // Parse weights
            for (i, weight_str) in weights.iter().enumerate() {
                let weight: i32 = weight_str.parse().map_err(|e| {
                    TpcdsError::new(&format!("Failed to parse weight '{}': {}", weight_str, e))
                })?;
                weights_builders[i].compute_and_add_next_weight(weight)?;
            }
        }

        let weights_lists = weights_builders
            .into_iter()
            .map(|builder| builder.build())
            .collect();

        Ok(CalendarDistribution {
            days_of_year,
            quarters,
            holiday_flags,
            weights_lists,
        })
    }

    /// Get the quarter for a given day index (1-based index)
    pub fn get_quarter_at_index(index: i32) -> i32 {
        let dist = Self::get_instance();
        dist.quarters[(index - 1) as usize]
    }

    /// Get the holiday flag for a given day index (1-based index)
    pub fn get_is_holiday_flag_at_index(index: i32) -> i32 {
        let dist = Self::get_instance();
        dist.holiday_flags[(index - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_distribution_loading() {
        let dist = CalendarDistribution::get_instance();
        assert!(dist.days_of_year.len() > 0);
        assert_eq!(dist.quarters.len(), dist.days_of_year.len());
        assert_eq!(dist.holiday_flags.len(), dist.days_of_year.len());
    }

    #[test]
    fn test_get_quarter_at_index() {
        // Day 1 (Jan 1) should be in Q1
        let quarter = CalendarDistribution::get_quarter_at_index(1);
        assert_eq!(quarter, 1);
    }

    #[test]
    fn test_days_before_month() {
        // Non-leap year
        assert_eq!(CalendarDistribution::DAYS_BEFORE_MONTH[0][0], 0);  // January
        assert_eq!(CalendarDistribution::DAYS_BEFORE_MONTH[0][1], 31); // February

        // Leap year
        assert_eq!(CalendarDistribution::DAYS_BEFORE_MONTH[1][2], 60); // March in leap year
    }
}
