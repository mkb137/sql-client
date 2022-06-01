use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum SqlConnectionAttestationProtocol {
    NotSpecified = 0,
    AAS = 1,
    None = 2,
    HGS = 3,
}

impl TryFrom<&str> for SqlConnectionAttestationProtocol {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "notspecified" => Ok(SqlConnectionAttestationProtocol::NotSpecified),
            "aas" => Ok(SqlConnectionAttestationProtocol::AAS),
            "none" => Ok(SqlConnectionAttestationProtocol::None),
            "hgs" => Ok(SqlConnectionAttestationProtocol::HGS),
            _ => {
                log::warn!("Unsupported attestation protocol {:?}", value);
                Err(SqlClientError::UnsupportedValue(
                    "SqlClientError".to_string(),
                    value.to_string(),
                ))
            }
        }
    }
}

impl Display for SqlConnectionAttestationProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlConnectionAttestationProtocol::NotSpecified => write!(f, "NotSpecified"),
            SqlConnectionAttestationProtocol::AAS => write!(f, "AAS"),
            SqlConnectionAttestationProtocol::None => write!(f, "None"),
            SqlConnectionAttestationProtocol::HGS => write!(f, "HGS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("NotSpecified", SqlConnectionAttestationProtocol::NotSpecified)]
    #[case("AAS", SqlConnectionAttestationProtocol::AAS)]
    #[case("None", SqlConnectionAttestationProtocol::None)]
    #[case("HGS", SqlConnectionAttestationProtocol::HGS)]
    fn test_from_string(#[case] value: &str, #[case] expected: SqlConnectionAttestationProtocol) {
        let actual: SqlConnectionAttestationProtocol = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(SqlConnectionAttestationProtocol::NotSpecified, "NotSpecified")]
    #[case(SqlConnectionAttestationProtocol::AAS, "AAS")]
    #[case(SqlConnectionAttestationProtocol::None, "None")]
    #[case(SqlConnectionAttestationProtocol::HGS, "HGS")]
    fn test_to_string(#[case] value: SqlConnectionAttestationProtocol, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
