use crate::SqlClientError;
use rand::Rng;
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

/// Internal methods implemented by retry intervals.
trait SqlRetryIntervalInternal {
    // The minimum time interval.
    fn min_time_interval_ref(&self) -> &Duration;
    // The maximum time interval.
    fn max_time_interval_ref(&self) -> &Duration;
    /// The current retry interval.
    fn current_ref(&self) -> &Duration;
    /// Sets the current retry interval
    fn set_current(&mut self, value: Duration);
    /// Gets the next interval
    fn next_interval(&mut self) -> Duration;
    /// Resets the enumerator
    fn reset_hidden(&mut self);
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

/// Implements SqlRetryInterval for any type that implements SqlRetryIntervalInternal
impl<S> SqlRetryInterval for S
where
    S: SqlRetryIntervalInternal + Clone,
{
    fn current(&self) -> Duration {
        self.current_ref().clone()
    }

    fn move_next(&mut self) -> bool {
        // If the current value is less than the maximum...
        if &self.current_ref() < &self.max_time_interval_ref() {
            // Get the next interval
            let next = self.next_interval();
            // If the next value is also less than the maximum...
            if &next <= &self.max_time_interval_ref() {
                // Save it as the new "current"
                self.set_current(next);
                // Return that we were able to get a new value
                true
            } else {
                // Return that we were not able to get a new value
                false
            }
        } else {
            // We're already over the maximum.  We can't go next.
            false
        }
    }

    fn reset(&mut self) {
        self.reset_hidden()
    }
}

#[derive(Clone)]
pub(crate) struct SqlExponentialIntervalEnumerator {
    internal_counter: u64,
    max_random: u64,
    min_random: u64,
    min_time_interval: Duration,
    max_time_interval: Duration,
    current: Duration,
}
impl SqlExponentialIntervalEnumerator {
    /// Creates a new enumerator with a specified amount of randomness.
    fn new_random(
        delta_backoff_time: Duration,
        max_time_interval: Duration,
        min_time_interval: Duration,
        randomness: f64,
    ) -> Result<Self, SqlClientError> {
        // Validate the intervals.
        validate(&delta_backoff_time, &max_time_interval, &min_time_interval)?;
        // Get the random range
        let (min_random, max_random) = get_random_interval(&delta_backoff_time, randomness);
        // Return the value
        Ok(Self {
            internal_counter: 1,
            max_random,
            min_random,
            max_time_interval,
            min_time_interval,
            current: Duration::new(0, 0),
        })
    }

    /// Creates a new enumerator.
    pub fn new(
        delta_backoff_time: Duration,
        max_time_interval: Duration,
        min_time_interval: Duration,
    ) -> Result<Self, SqlClientError> {
        Self::new_random(
            delta_backoff_time,
            max_time_interval,
            min_time_interval,
            0.2,
        )
    }
}
impl SqlRetryIntervalInternal for SqlExponentialIntervalEnumerator {
    fn min_time_interval_ref(&self) -> &Duration {
        &self.min_time_interval
    }

    fn max_time_interval_ref(&self) -> &Duration {
        &self.max_time_interval
    }

    fn current_ref(&self) -> &Duration {
        &self.current
    }

    fn set_current(&mut self, value: Duration) {
        self.current = value
    }

    /// Gets the next interval.
    fn next_interval(&mut self) -> Duration {
        log::debug!("next_interval");
        // Get the exponent
        let exponent = &self.internal_counter.pow(2) - 1;
        log::debug!(
            " - internal_counter = {:?}, exponent = {:?}",
            &self.internal_counter,
            exponent
        );
        // Get an amount of randomness in the range
        let random = get_random(self.min_random, self.max_random);
        log::debug!(" - random = {:?}", random);
        // Get the delta, using an exponent of our counter times the mildly randomized gap.
        let delta = exponent * random;
        log::debug!(" - delta = {:?}", delta);
        // Get the new time
        let new_time = self.min_time_interval + Duration::from_millis(delta);
        log::debug!(" - returning interval = {:?}", new_time);
        // Update the internal counter
        self.internal_counter += 1;
        // Return the new time
        new_time
    }

    fn reset_hidden(&mut self) {
        self.current = Duration::new(0, 0);
        self.internal_counter = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry_enumerators2::SqlRetryInterval;
    use std::cmp::min;

    #[test]
    pub fn test_exponential_interval_enumerator() {
        let gap_threshold = Duration::from_secs(1);
        let min_interval = Duration::from_secs(5);
        let max_interval = Duration::from_secs(20);
        // Create the enumerator, eliminating the randomness so that our tests are repeatable.
        let mut subject = SqlExponentialIntervalEnumerator::new_random(
            gap_threshold,
            max_interval,
            min_interval,
            0.0,
        )
        .unwrap();
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the minimum
        assert_eq!(5, subject.current().as_secs());
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should have advanced by the next exponent of the gap
        assert_eq!(8, subject.current().as_secs());
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should have advanced by the next exponent of the gap
        assert_eq!(13, subject.current().as_secs());
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should have advanced by the next exponent of the gap
        assert_eq!(20, subject.current().as_secs());
        // Move to the next interval, which should return "false".
        assert!(!subject.move_next());
        // The current value should NOT have advanced
        assert_eq!(20, subject.current().as_secs());
        // Reset the intervals
        subject.reset();
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the minimum
        assert_eq!(5, subject.current().as_secs());
    }
}
