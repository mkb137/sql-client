use super::validate;
use crate::retry_enumerators::{get_random, get_random_interval};
use crate::SqlClientError;
use rand::Rng;
use std::cmp::min;
use std::ops::Deref;
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct SqlFixedIntervalEnumerator {
    max_random: u64,
    min_random: u64,
    current: Duration,
}
impl SqlFixedIntervalEnumerator {
    /// Creates a new enumerator with a specified amount of randomness.
    fn new_random(delta_backoff_time: Duration, randomness: f64) -> Result<Self, SqlClientError> {
        // Get the random range
        let (min_random, max_random) = get_random_interval(&delta_backoff_time, randomness);
        // Return the value
        Ok(Self {
            max_random,
            min_random,
            current: Duration::new(0, 0),
        })
    }

    /// Creates a new enumerator.
    pub fn new(delta_backoff_time: Duration) -> Result<Self, SqlClientError> {
        Self::new_random(delta_backoff_time, 0.2)
    }

    /// Gets the next interval.
    fn next_interval(&mut self) -> Duration {
        log::debug!("next_interval");
        // Get a random amount of time between the min and max.
        let random = get_random(self.min_random, self.max_random);
        log::debug!(" - random = {:?}", random);
        // Return the new time
        Duration::from_millis(random)
    }
}

impl super::SqlRetryInterval for SqlFixedIntervalEnumerator {
    /// Gets the current value of the retry interval.
    fn current(&self) -> Duration {
        self.current.clone()
    }

    /// Moves to the next retry interval.
    fn move_next(&mut self) -> bool {
        // Get the next interval
        let next = self.next_interval();
        // Save it as the new "current"
        self.current = next;
        // This enumerator is always able to get a new value.
        true
    }

    /// Resets the retry interval.
    fn reset(&mut self) {
        // Reset "current" to the minimum
        self.current = Duration::new(0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry_enumerators::SqlRetryInterval;
    use std::cmp::min;

    #[test]
    pub fn test_fixed_interval_enumerator() {
        let gap_threshold = Duration::from_secs(1);
        // Create the enumerator, eliminating the randomness so that our tests are repeatable.
        let mut subject = SqlFixedIntervalEnumerator::new_random(gap_threshold, 0.0).unwrap();
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the fixed gap
        assert_eq!(1, subject.current().as_secs());
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the fixed gap
        assert_eq!(1, subject.current().as_secs());
        // Reset the intervals
        subject.reset();
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the fixed gap
        assert_eq!(1, subject.current().as_secs());
    }
}
