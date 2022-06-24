mod sql_exponential_interval_enumerator;
mod sql_fixed_interval_enumerator;
mod sql_incremental_interval_enumerator;
mod sql_none_interval_enumerator;

use crate::SqlClientError;
use rand::Rng;
use std::cmp::min;
use std::ops::Deref;
use std::time::Duration;

/// The minimum allowable min, max, or gap interval.
const MIN_DURATION: Duration = Duration::new(0, 0);
/// The maximum allowable min, max, or gap interval.
const MAX_DURATION: Duration = Duration::from_secs(120);

/// Ensures an interval is between the max and min
fn ensure_in_range(
    min_time_interval: &Duration,
    max_time_interval: &Duration,
    time_interval: Duration,
) -> Duration {
    log::debug!(
        "ensure_in_range - min: {:?}, max: {:?}, value: {:?}",
        min_time_interval,
        max_time_interval,
        time_interval
    );
    // Get a value guarded against the max
    let guarded_max =
        // If the value is less than or equal to the max (i.e. is in range)...
        if &time_interval <= max_time_interval {
            // Use it as-is
            time_interval
        }
        // Otherwise, fuzz it by a random amount
        else {
            let millis = rand::random::<f64>() * (max_time_interval.as_millis() as f64 * 0.2) + (max_time_interval.as_millis() as f64 * 0.8);
            Duration::from_millis(millis as u64)
        };
    // Return the value guarded against the min
    if &guarded_max < min_time_interval {
        min_time_interval.clone()
    } else {
        guarded_max
    }
}

/// Gets a range of values on either side of the gap time interval
fn get_random_interval(gap_time_interval: &Duration, randomness: f64) -> (u64, u64) {
    // Figure out the randoms
    let temp_max = (gap_time_interval.as_millis() as f64) * (1.0 + randomness);
    let temp_min = (gap_time_interval.as_millis() as f64) * (1.0 - randomness);
    log::debug!(" - temp_max = {:?}, temp_min = {:?}", temp_max, temp_min);
    let u64_max = u64::MAX as f64;
    let max_random = if temp_max > u64_max {
        log::debug!(" - temp_max exceeds u64 max");
        u64::MAX
    } else {
        temp_max.round() as u64
    };
    let min_random = if temp_min > u64_max {
        log::debug!(" - temp_min exceeds u64 max");
        (u64::MIN as f64 * 0.6) as u64
    } else {
        temp_min.round() as u64
    };
    log::debug!(
        " - max_random = {:?}, min_random = {:?}",
        max_random,
        min_random
    );
    // Return the range
    (min_random, max_random)
}

// Gets a random value in the range.
fn get_random(min_random: u64, max_random: u64) -> u64 {
    // If we've eliminated randomness (as in unit testing)...
    if max_random == min_random {
        // Return either value.
        min_random
    } else {
        // Create a random number generator
        let mut rng = rand::thread_rng();
        log::debug!(" - created rng");
        // Get an amount of randomness within the range
        rng.gen_range(min_random..max_random)
    }
}
/// Validates the intervals.
fn validate(
    gap_time_interval: &Duration,
    max_time_interval: &Duration,
    min_time_interval: &Duration,
) -> Result<(), SqlClientError> {
    if min_time_interval < &MIN_DURATION || max_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange(
            "min_time_interval".to_string(),
            format!(
                "min_time_interval must be between {:?} and {:?}",
                MIN_DURATION, MAX_DURATION
            )
            .to_string(),
        ))
    } else if max_time_interval < &MIN_DURATION || max_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange(
            "max_time_interval".to_string(),
            format!(
                "max_time_interval must be between {:?} and {:?}",
                MIN_DURATION, MAX_DURATION
            )
            .to_string(),
        ))
    } else if gap_time_interval < &MIN_DURATION || gap_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange(
            "gap_time_interval".to_string(),
            format!(
                "gap_time_interval must be between {:?} and {:?}",
                MIN_DURATION, MAX_DURATION
            )
            .to_string(),
        ))
    } else if max_time_interval < min_time_interval {
        Err(SqlClientError::ArgumentOutOfRange(
            "max_time_interval".to_string(),
            "max_time_interval must be greater than min_time_interval".to_string(),
        ))
    } else {
        // Validation succeeded.
        Ok(())
    }
}

/// A SQL retry interval.
///
/// Generates retry intervals for use when retrying a connection.
pub(crate) trait SqlRetryInterval: Clone {
    /// The current retry interval.
    fn current(&self) -> Duration;
    /// Moved "current" to the next retry interval.
    fn move_next(&mut self) -> bool;
    /// Resets the retry interval.
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry_enumerators::SqlRetryInterval;
    use std::cmp::min;

    // Test "ensure in range" for <= max values
    #[rstest::rstest]
    #[case(10, 20, 15, 15, true)]
    #[case(10, 20, 10, 10, true)]
    #[case(10, 20, 9, 10, true)]
    #[case(10, 20, 20, 20, true)]
    #[case(10, 20, 21, 99, false)]
    fn test_ensure_in_range(
        #[case] min: u64,
        #[case] max: u64,
        #[case] value: u64,
        #[case] expected: u64,
        #[case] exact: bool,
    ) {
        let min_duration = Duration::from_secs(min);
        let max_duration = Duration::from_secs(max);
        let input_duration = Duration::from_secs(value);
        let actual_duration = ensure_in_range(&min_duration, &max_duration, input_duration);
        // If we're looking for an exact match (which we do when the value is less than or equal the maximum)...
        if exact {
            assert_eq!(expected, actual_duration.as_secs());
        }
        // We can't match exactly since a degree of randomness is used.  Just check that the value is between the min and max.
        else {
            assert!(actual_duration.as_secs() >= min);
            assert!(actual_duration.as_secs() <= max);
        }
    }
}
