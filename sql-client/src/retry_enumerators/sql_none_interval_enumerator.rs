use super::validate;
use crate::retry_enumerators::{get_random, get_random_interval};
use crate::SqlClientError;
use rand::Rng;
use std::cmp::min;
use std::ops::Deref;
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct SqlNoneIntervalEnumerator;
impl SqlNoneIntervalEnumerator {
    /// Creates a new enumerator.
    pub fn new() -> Result<Self, SqlClientError> {
        Ok(Self)
    }
}

impl super::SqlRetryInterval for SqlNoneIntervalEnumerator {
    /// Gets the current value of the retry interval.
    fn current(&self) -> Duration {
        Duration::new(0, 0)
    }

    /// Moves to the next retry interval.
    fn move_next(&mut self) -> bool {
        // This enumerator is always able to get a new value.
        true
    }

    /// Resets the retry interval.
    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry_enumerators::SqlRetryInterval;
    use std::cmp::min;

    #[test]
    pub fn test_none_interval_enumerator() {
        // Create the enumerator, eliminating the randomness so that our tests are repeatable.
        let mut subject = SqlNoneIntervalEnumerator::new().unwrap();
        // Move to the next interval, which should return "true".
        assert!(subject.move_next());
        // The current value should be the 0
        assert_eq!(0, subject.current().as_secs());
    }
}
