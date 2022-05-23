use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

/// ?
#[derive(PartialEq, Debug)]
pub enum PoolBlockingPeriod {
    // Blocking period OFF for Azure SQL servers, but ON for all other SQL servers.
    Auto = 0,
    // Blocking period ON for all SQL servers including Azure SQL servers.
    AlwaysBlock = 1,
    // Blocking period OFF for all SQL servers including Azure SQL servers.
    NeverBlock = 2,
}
impl TryFrom<&str> for PoolBlockingPeriod {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "auto" => Ok(PoolBlockingPeriod::Auto),
            "alwaysblock" => Ok(PoolBlockingPeriod::AlwaysBlock),
            "neverblock" => Ok(PoolBlockingPeriod::NeverBlock),
            _ => Err(SqlClientError::UnsupportedValue(
                "Pool blocking period".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl Display for PoolBlockingPeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolBlockingPeriod::Auto => write!(f, "Auto"),
            PoolBlockingPeriod::AlwaysBlock => write!(f, "AlwaysBlock"),
            PoolBlockingPeriod::NeverBlock => write!(f, "NeverBlock"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("Auto", PoolBlockingPeriod::Auto)]
    #[case("auto", PoolBlockingPeriod::Auto)]
    #[case("AlwaysBlock", PoolBlockingPeriod::AlwaysBlock)]
    #[case("alwaysblock", PoolBlockingPeriod::AlwaysBlock)]
    #[case("NeverBlock", PoolBlockingPeriod::NeverBlock)]
    #[case("neverblock", PoolBlockingPeriod::NeverBlock)]
    fn test_from_string(#[case] value: &str, #[case] expected: PoolBlockingPeriod) {
        let actual: PoolBlockingPeriod = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(PoolBlockingPeriod::Auto, "Auto")]
    #[case(PoolBlockingPeriod::AlwaysBlock, "AlwaysBlock")]
    #[case(PoolBlockingPeriod::NeverBlock, "NeverBlock")]
    fn test_to_string(#[case] value: PoolBlockingPeriod, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
