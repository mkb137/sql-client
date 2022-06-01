use crate::SqlClientError;
use std::fmt::{Display, Formatter};

pub(crate) struct TransactionBindingKeywords;
impl TransactionBindingKeywords {
    pub const IMPLICIT_UNBIND: &'static str = "Implicit Unbind";
    pub const EXPLICIT_UNBIND: &'static str = "Explicit Unbind";
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum TransactionBinding {
    ImplicitUnbind,
    ExplicitUnbind,
}

impl TryFrom<&str> for TransactionBinding {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "implicit unbind" => Ok(TransactionBinding::ImplicitUnbind),
            "explicit unbind" => Ok(TransactionBinding::ExplicitUnbind),
            _ => {
                log::warn!("Unsupported transaction binding {:?}", value);
                Err(SqlClientError::UnsupportedValue(
                    "Transaction Binding".to_string(),
                    value.to_string(),
                ))
            }
        }
    }
}

impl Display for TransactionBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionBinding::ImplicitUnbind => {
                write!(f, "{}", TransactionBindingKeywords::IMPLICIT_UNBIND)
            }
            TransactionBinding::ExplicitUnbind => {
                write!(f, "{}", TransactionBindingKeywords::EXPLICIT_UNBIND)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("implicit Unbind", TransactionBinding::ImplicitUnbind)]
    #[case("Implicit Unbind", TransactionBinding::ImplicitUnbind)]
    #[case("explicit Unbind", TransactionBinding::ExplicitUnbind)]
    #[case("Explicit Unbind", TransactionBinding::ExplicitUnbind)]
    fn test_from_string(#[case] value: &str, #[case] expected: TransactionBinding) {
        let actual: TransactionBinding = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(TransactionBinding::ImplicitUnbind, "Implicit Unbind")]
    #[case(TransactionBinding::ExplicitUnbind, "Explicit Unbind")]
    fn test_to_string(#[case] value: TransactionBinding, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
