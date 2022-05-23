use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

/// ?
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SqlConnectionColumnEncryptionSetting {
    Disabled = 0,
    Enabled = 1,
}

impl TryFrom<&str> for SqlConnectionColumnEncryptionSetting {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "disabled" => Ok(SqlConnectionColumnEncryptionSetting::Disabled),
            "enabled" => Ok(SqlConnectionColumnEncryptionSetting::Enabled),
            _ => Err(SqlClientError::UnsupportedValue(
                "SqlConnectionColumnEncryptionSetting".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl Display for SqlConnectionColumnEncryptionSetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlConnectionColumnEncryptionSetting::Disabled => write!(f, "Disabled"),
            SqlConnectionColumnEncryptionSetting::Enabled => write!(f, "Enabled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("Disabled", SqlConnectionColumnEncryptionSetting::Disabled)]
    #[case("disabled", SqlConnectionColumnEncryptionSetting::Disabled)]
    #[case("Enabled", SqlConnectionColumnEncryptionSetting::Enabled)]
    #[case("enabled", SqlConnectionColumnEncryptionSetting::Enabled)]
    fn test_from_string(
        #[case] value: &str,
        #[case] expected: SqlConnectionColumnEncryptionSetting,
    ) {
        let actual: SqlConnectionColumnEncryptionSetting = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(SqlConnectionColumnEncryptionSetting::Disabled, "Disabled")]
    #[case(SqlConnectionColumnEncryptionSetting::Enabled, "Enabled")]
    fn test_to_string(#[case] value: SqlConnectionColumnEncryptionSetting, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
