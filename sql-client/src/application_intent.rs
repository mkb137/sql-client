use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub(crate) enum ApplicationIntent {
    /// The application workload type when connecting to a server is read write.
    ReadWrite = 0,
    /// The application workload type when connecting to a server is read only.
    ReadOnly = 1,
}

impl TryFrom<&str> for ApplicationIntent {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "readonly" => Ok(ApplicationIntent::ReadOnly),
            "readwrite" => Ok(ApplicationIntent::ReadWrite),
            _ => {
                log::warn!("Unsupported application intent {:?}", value);
                Err(SqlClientError::UnsupportedValue(
                    "SqlClientError".to_string(),
                    value.to_string(),
                ))
            }
        }
    }
}

impl Display for ApplicationIntent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationIntent::ReadWrite => write!(f, "ReadWrite"),
            ApplicationIntent::ReadOnly => write!(f, "ReadOnly"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("readwrite", ApplicationIntent::ReadWrite)]
    #[case("ReadWrite", ApplicationIntent::ReadWrite)]
    #[case("readonly", ApplicationIntent::ReadOnly)]
    #[case("ReadOnly", ApplicationIntent::ReadOnly)]
    fn test_from_string(#[case] value: &str, #[case] expected: ApplicationIntent) {
        let actual: ApplicationIntent = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(ApplicationIntent::ReadWrite, "ReadWrite")]
    #[case(ApplicationIntent::ReadOnly, "ReadOnly")]
    fn test_to_string(#[case] value: ApplicationIntent, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
