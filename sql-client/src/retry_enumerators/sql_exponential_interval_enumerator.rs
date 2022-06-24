use super::validate;
use crate::retry_enumerators::{get_random, get_random_interval};
use crate::SqlClientError;
use rand::Rng;
use std::cmp::min;
use std::ops::Deref;
use std::time::Duration;

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
}

impl super::SqlRetryInterval for SqlExponentialIntervalEnumerator {
    /// Gets the current value of the retry interval.
    fn current(&self) -> Duration {
        self.current.clone()
    }

    /// Moves to the next retry interval.
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

    /// Resets the retry interval.
    fn reset(&mut self) {
        // Reset "current" to the minimum
        self.current = Duration::new(0, 0);
        // Reset the internal counter.
        self.internal_counter = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry_enumerators::SqlRetryInterval;
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
