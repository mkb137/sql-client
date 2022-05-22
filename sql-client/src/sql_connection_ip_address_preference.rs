use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

/// ?
#[derive(PartialEq, Debug)]
pub(crate) enum SqlConnectionIpAddressPreference {
    IPv4First = 0, // default
    IPv6First = 1,
    UsePlatformDefault = 2,
}

impl TryFrom<&str> for SqlConnectionIpAddressPreference {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "ipv4first" => Ok(SqlConnectionIpAddressPreference::IPv4First),
            "ipv6first" => Ok(SqlConnectionIpAddressPreference::IPv6First),
            "useplatformdefault" => Ok(SqlConnectionIpAddressPreference::UsePlatformDefault),
            _ => Err(SqlClientError::UnsupportedValue(
                "IP Address Preference".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl Display for SqlConnectionIpAddressPreference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlConnectionIpAddressPreference::IPv4First => write!(f, "IPv4First"),
            SqlConnectionIpAddressPreference::IPv6First => write!(f, "IPv6First"),
            SqlConnectionIpAddressPreference::UsePlatformDefault => write!(f, "UsePlatformDefault"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("IPv4First", SqlConnectionIpAddressPreference::IPv4First)]
    #[case("IPv4First", SqlConnectionIpAddressPreference::IPv4First)]
    #[case("IPv6First", SqlConnectionIpAddressPreference::IPv6First)]
    #[case("IPv6First", SqlConnectionIpAddressPreference::IPv6First)]
    #[case(
        "UsePlatformDefault",
        SqlConnectionIpAddressPreference::UsePlatformDefault
    )]
    #[case(
        "UsePlatformDefault",
        SqlConnectionIpAddressPreference::UsePlatformDefault
    )]
    fn test_from_string(#[case] value: &str, #[case] expected: SqlConnectionIpAddressPreference) {
        let actual: SqlConnectionIpAddressPreference = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(SqlConnectionIpAddressPreference::IPv4First, "IPv4First")]
    #[case(SqlConnectionIpAddressPreference::IPv6First, "IPv6First")]
    #[case(
        SqlConnectionIpAddressPreference::UsePlatformDefault,
        "UsePlatformDefault"
    )]
    fn test_to_string(#[case] value: SqlConnectionIpAddressPreference, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
