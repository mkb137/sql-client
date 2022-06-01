use crate::SqlClientError;
use crate::TypeSystem::SqlServer2008;
use std::fmt::{Display, Formatter};

pub(crate) struct TypeSystemVersion;
impl TypeSystemVersion {
    pub const LATEST: &'static str = "LATEST";
    pub const SQL_SERVER_2000: &'static str = "SQL Server 2000";
    pub const SQL_SERVER_2005: &'static str = "SQL Server 2005";
    pub const SQL_SERVER_2008: &'static str = "SQL Server 2008";
    pub const SQL_SERVER_2012: &'static str = "SQL Server 2012";
}

#[derive(PartialEq, Debug, Clone, Copy)]
/// SQL Type System constants.
pub(crate) enum TypeSystem {
    SqlServer2000 = 2000,
    SqlServer2005 = 2005,
    SqlServer2008 = 2008,
    SqlServer2012 = 2012,
}
impl TypeSystem {
    pub const LATEST: TypeSystem = SqlServer2008;
}

impl TryFrom<&str> for TypeSystem {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "sql server 2000" => Ok(TypeSystem::SqlServer2000),
            "sql server 2005" => Ok(TypeSystem::SqlServer2005),
            "sql server 2008" => Ok(TypeSystem::SqlServer2008),
            "sql server 2012" => Ok(TypeSystem::SqlServer2012),
            _ => {
                log::warn!("Unsupported type system version {:?}", value);
                Err(SqlClientError::UnsupportedValue(
                    "Type System".to_string(),
                    value.to_string(),
                ))
            }
        }
    }
}

impl Display for TypeSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSystem::SqlServer2000 => {
                write!(f, "{}", TypeSystemVersion::SQL_SERVER_2000)
            }
            TypeSystem::SqlServer2005 => {
                write!(f, "{}", TypeSystemVersion::SQL_SERVER_2005)
            }
            TypeSystem::SqlServer2008 => {
                write!(f, "{}", TypeSystemVersion::SQL_SERVER_2008)
            }
            TypeSystem::SqlServer2012 => {
                write!(f, "{}", TypeSystemVersion::SQL_SERVER_2012)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("SQL Server 2000", TypeSystem::SqlServer2000)]
    #[case("SQL Server 2005", TypeSystem::SqlServer2005)]
    #[case("SQL Server 2008", TypeSystem::SqlServer2008)]
    #[case("SQL Server 2012", TypeSystem::SqlServer2012)]
    fn test_from_string(#[case] value: &str, #[case] expected: TypeSystem) {
        let actual: TypeSystem = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(TypeSystem::SqlServer2000, "SQL Server 2000")]
    #[case(TypeSystem::SqlServer2005, "SQL Server 2005")]
    #[case(TypeSystem::SqlServer2008, "SQL Server 2008")]
    #[case(TypeSystem::SqlServer2012, "SQL Server 2012")]
    fn test_to_string(#[case] value: TypeSystem, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
