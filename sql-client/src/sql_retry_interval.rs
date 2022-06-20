use std::cmp::min;
use std::ops::Deref;
use std::time::Duration;
use rand::Rng;
use crate::SqlClientError;

/// The minimum allowable min, max, or gap interval.
const MIN_DURATION: Duration = Duration::new(0,0);
/// The maximum allowable min, max, or gap interval.
const MAX_DURATION: Duration = Duration::from_secs(120);

/// Validates the intervals.
fn validate(gap_time_interval: &Duration, max_time_interval: &Duration, min_time_interval: &Duration) -> Result<(),SqlClientError> {
    if min_time_interval < &MIN_DURATION || max_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange("min_time_interval".to_string(), format!("min_time_interval must be between {:?} and {:?}", MIN_DURATION, MAX_DURATION).to_string()))
    } else if max_time_interval < &MIN_DURATION || max_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange("max_time_interval".to_string(), format!("max_time_interval must be between {:?} and {:?}", MIN_DURATION, MAX_DURATION).to_string()))
    } else if gap_time_interval < &MIN_DURATION || gap_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange("gap_time_interval".to_string(), format!("gap_time_interval must be between {:?} and {:?}", MIN_DURATION, MAX_DURATION).to_string()))
    } else if max_time_interval < min_time_interval{
        Err(SqlClientError::ArgumentOutOfRange("max_time_interval".to_string(), "max_time_interval must be greater than min_time_interval".to_string()))
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

#[derive(Clone)]
pub(crate) struct SqlExponentialIntervalEnumerator {
    internal_counter: u64,
    max_random: u64,
    min_random: u64,
    delta_backoff_time: Duration,
    min_time_interval: Duration,
    max_time_interval: Duration,
    current: Duration
}
impl SqlExponentialIntervalEnumerator {

    /// Creates a new enumerator with a specified amount of randomness.
    fn new_random(delta_backoff_time: Duration, max_time_interval: Duration, min_time_interval: Duration, randomness: f64) -> Result<Self,SqlClientError> {
        // Validate the intervals.
        validate(&delta_backoff_time,&max_time_interval,&min_time_interval)?;
        // Figure out the randoms
        let temp_max = (delta_backoff_time.as_millis() as f64) * (1.0 + randomness);
        let temp_min = (delta_backoff_time.as_millis() as f64) * (1.0 - randomness);
        log::debug!(" - temp_max = {:?}, temp_min = {:?}", temp_max, temp_min);
        let u64_max = u64::MAX as f64;
        let max_random =
            if temp_max > u64_max {
                log::debug!(" - temp_max exceeds u64 max");
                u64::MAX
            } else {
                temp_max.round() as u64
            };
        let min_random =
            if temp_min > u64_max {
                log::debug!(" - temp_min exceeds u64 max");
                (u64::MIN as f64 * 0.6) as u64
            } else {
                temp_min.round() as u64
            };
        log::debug!(" - max_random = {:?}, min_random = {:?}", max_random, min_random);
        // Return the value
        Ok(Self {
            internal_counter: 1,
            max_random,
            min_random,
            delta_backoff_time,
            max_time_interval,
            min_time_interval,
            current: Duration::new(0,0)
        })
    }

    /// Creates a new enumerator.
    pub fn new(delta_backoff_time: Duration, max_time_interval: Duration, min_time_interval: Duration) -> Result<Self,SqlClientError> {
        Self::new_random(delta_backoff_time, max_time_interval, min_time_interval, 0.2)
    }

    /// Gets the next interval.
    fn next_interval(&mut self) -> Duration {
        log::debug!("next_interval");
        // Get the exponent
        let exponent = &self.internal_counter.pow(2) - 1;
        log::debug!(" - internal_counter = {:?}, exponent = {:?}",  &self.internal_counter, exponent);
        // Get an amount of randomness in the range
        let random =
            // If we've eliminated randomness (as in unit testing)...
            if self.max_random == self.min_random {
                // Return either value.
                self.min_random
            } else {
                // Create a random number generator
                let mut rng = rand::thread_rng();
                log::debug!(" - created rng");
                // Get an amount of randomness within the range
                rng.gen_range(self.min_random..self.max_random)
            };
        log::debug!(" - random = {:?}", random);
        // Get the delta, using an exponent of our counter times the mildly randomized gap.
        let delta = exponent * random;
        log::debug!(" - delta = {:?}", delta);
        // Get the new time
        let new_time = self.min_time_interval + Duration::from_millis(delta);
        log::debug!(" - returning interval = {:?}", new_time);
        // Update the internal counter
        self.internal_counter+= 1;
        // Return the new time
        new_time
    }
}

impl SqlRetryInterval for SqlExponentialIntervalEnumerator {
    fn current(&self) -> Duration {
        self.current.clone()
    }

    fn move_next(&mut self) -> bool {
        // If the current value is less than the maximum...
        if &self.current < &self.max_time_interval {
            // Get the next interval
            let next = self.next_interval();
            // If the next value is also less than the maximum...
            if &next <= &self.max_time_interval {
                // Save it as the new "current"
                self.current = next;
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
        // Reset the internal counter.
        self.internal_counter = 1;
    }
}

#[cfg(test)]
mod tests  {
    use std::cmp::min;
    use super::*;

    #[test]
    pub fn test_exponential_interval_enumerator() {
        let gap_threshold = Duration::from_secs(1);
        let min_interval = Duration::from_secs(5);
        let max_interval = Duration::from_secs(20);
        // Create the enumerator, eliminating the randomness so that our tests are repeatable.
        let mut subject = SqlExponentialIntervalEnumerator::new_random(gap_threshold, max_interval, min_interval, 0.0).unwrap();
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
    }

}