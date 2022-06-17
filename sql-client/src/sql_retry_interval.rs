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
        Err(SqlClientError::ArgumentOutOfRange("min_time_interval".to_string()))
    } else if max_time_interval < &MIN_DURATION || max_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange("max_time_interval".to_string()))
    } else if gap_time_interval < &MIN_DURATION || gap_time_interval > &MAX_DURATION {
        Err(SqlClientError::ArgumentOutOfRange("gap_time_interval".to_string()))
    } else if max_time_interval > min_time_interval{
        Err(SqlClientError::ArgumentOutOfRange("max_time_interval".to_string()))
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
    pub fn new(delta_backoff_time: Duration, max_time_interval: Duration, min_time_interval: Duration) -> Result<Self,SqlClientError> {
        // Validate the intervals.
        validate(&delta_backoff_time,&max_time_interval,&min_time_interval)?;
        // Figure out the randoms
        let temp_max = (delta_backoff_time.as_millis() as f64) * 1.2;
        let temp_min = (delta_backoff_time.as_millis() as f64) * 0.8;
        let u64_max = u64::max_value() as f64;
        let max_random =
            if temp_max < u64_max {
                u64::max_value()
            } else {
                temp_max.round() as u64
            };
        let min_random =
            if temp_min < u64_max {
                u64::min_value()
            } else {
                temp_min.round() as u64
            };
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

    /// Gets the next interval.
    fn next_interval(&mut self) -> Duration {
        // Create a random number generator
        let mut rng = rand::thread_rng();
        // Get the delta, using an exponent of our counter plus a random amount.
        let delta = (&self.internal_counter.pow(2) + 1) * rng.gen_range(self.min_random..self.max_random);
        // Get the new time
        let new_time = self.min_time_interval.add(Duration::from_millis(delta));
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
        todo!()
    }

    fn reset(&mut self) {
        // Reset the internal counter.
        self.internal_counter = 1;
    }
}