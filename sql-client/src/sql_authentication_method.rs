use crate::sql_client_error::SqlClientError;
use std::fmt::{Display, Formatter};

/// Describes the different SQL authentication methods that can be used by a client connecting to Azure SQL Database
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SqlAuthenticationMethod {
    /// The authentication method is not specified.
    NotSpecified,
    /// The authentication method is Sql Password.
    SqlPassword,
    /// The authentication method uses Active Directory Password. Use Active Directory Password to connect to a SQL Database using an Azure AD principal name and password.
    ActiveDirectoryPassword,
    /// The authentication method uses Active Directory Integrated. Use Active Directory Integrated to connect to a SQL Database using integrated Windows authentication.
    ActiveDirectoryIntegrated,
    /// The authentication method uses Active Directory Interactive. Available since .NET Framework 4.7.2.
    ActiveDirectoryInteractive,
    ActiveDirectoryServicePrincipal,
    ActiveDirectoryDeviceCodeFlow,
    ActiveDirectoryManagedIdentity,
    ActiveDirectoryMSI,
    ActiveDirectoryDefault,
    SqlCertificate,
}

impl TryFrom<&str> for SqlAuthenticationMethod {
    type Error = SqlClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "sql password" | "sqlpassword" => Ok(SqlAuthenticationMethod::SqlPassword),
            "active directory password" | "activedirectorypassword" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryPassword)
            }
            "active directory managed identity" | "activedirectorymanagedidentity" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryManagedIdentity)
            }
            "active directory integrated" | "activedirectoryintegrated" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryIntegrated)
            }
            "active directory interactive" | "activedirectoryinteractive" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryInteractive)
            }
            "active directory service principal" | "activedirectoryserviceprincipal" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryServicePrincipal)
            }
            "active directory device code flow" | "activedirectorydevicecodeflow" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow)
            }
            "active directory msi" | "activedirectorymsi" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryMSI)
            }
            "active directory default" | "activedirectorydefault" => {
                Ok(SqlAuthenticationMethod::ActiveDirectoryDefault)
            }
            "sql certificate" | "sqlcertificate" => Ok(SqlAuthenticationMethod::SqlCertificate),
            _ => Err(SqlClientError::UnsupportedValue(
                "Authentication Method".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl Display for SqlAuthenticationMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlAuthenticationMethod::NotSpecified => write!(f, "Not Specified"),
            SqlAuthenticationMethod::SqlPassword => write!(f, "Sql Password"),
            SqlAuthenticationMethod::ActiveDirectoryPassword => {
                write!(f, "Active Directory Password")
            }
            SqlAuthenticationMethod::ActiveDirectoryIntegrated => {
                write!(f, "Active Directory Integrated")
            }
            SqlAuthenticationMethod::ActiveDirectoryInteractive => {
                write!(f, "Active Directory Interactive")
            }
            SqlAuthenticationMethod::ActiveDirectoryServicePrincipal => {
                write!(f, "Active Directory Service Principal")
            }
            SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow => {
                write!(f, "Active Directory Device Code Flow")
            }
            SqlAuthenticationMethod::ActiveDirectoryManagedIdentity => {
                write!(f, "Active Directory Managed Identity")
            }
            SqlAuthenticationMethod::ActiveDirectoryMSI => write!(f, "Active Directory MSI"),
            SqlAuthenticationMethod::ActiveDirectoryDefault => {
                write!(f, "Active Directory Default")
            }
            SqlAuthenticationMethod::SqlCertificate => write!(f, "Sql Certificate"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("SqlPassword", SqlAuthenticationMethod::SqlPassword)]
    #[case("Sql Password", SqlAuthenticationMethod::SqlPassword)]
    #[case(
        "ActiveDirectoryManagedIdentity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "Active Directory Managed Identity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "ActiveDirectoryIntegrated",
        SqlAuthenticationMethod::ActiveDirectoryIntegrated
    )]
    #[case(
        "Active Directory Integrated",
        SqlAuthenticationMethod::ActiveDirectoryIntegrated
    )]
    #[case(
        "ActiveDirectoryInteractive",
        SqlAuthenticationMethod::ActiveDirectoryInteractive
    )]
    #[case(
        "Active Directory Interactive",
        SqlAuthenticationMethod::ActiveDirectoryInteractive
    )]
    #[case(
        "ActiveDirectoryServicePrincipal",
        SqlAuthenticationMethod::ActiveDirectoryServicePrincipal
    )]
    #[case(
        "Active Directory Service Principal",
        SqlAuthenticationMethod::ActiveDirectoryServicePrincipal
    )]
    #[case(
        "ActiveDirectoryDeviceCodeFlow",
        SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow
    )]
    #[case(
        "Active Directory Device Code Flow",
        SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow
    )]
    #[case(
        "ActiveDirectoryManagedIdentity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "Active Directory Managed Identity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case("ActiveDirectoryMSI", SqlAuthenticationMethod::ActiveDirectoryMSI)]
    #[case("Active Directory MSI", SqlAuthenticationMethod::ActiveDirectoryMSI)]
    #[case(
        "ActiveDirectoryDefault",
        SqlAuthenticationMethod::ActiveDirectoryDefault
    )]
    #[case(
        "Active Directory Default",
        SqlAuthenticationMethod::ActiveDirectoryDefault
    )]
    #[case("SqlCertificate", SqlAuthenticationMethod::SqlCertificate)]
    #[case("Sql Certificate", SqlAuthenticationMethod::SqlCertificate)]
    fn test_from_string(#[case] value: &str, #[case] expected: SqlAuthenticationMethod) {
        let actual: SqlAuthenticationMethod = value.try_into().unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(SqlAuthenticationMethod::SqlPassword, "Sql Password")]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity,
        "Active Directory Managed Identity"
    )]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryIntegrated,
        "Active Directory Integrated"
    )]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryInteractive,
        "Active Directory Interactive"
    )]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryServicePrincipal,
        "Active Directory Service Principal"
    )]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow,
        "Active Directory Device Code Flow"
    )]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity,
        "Active Directory Managed Identity"
    )]
    #[case(SqlAuthenticationMethod::ActiveDirectoryMSI, "Active Directory MSI")]
    #[case(
        SqlAuthenticationMethod::ActiveDirectoryDefault,
        "Active Directory Default"
    )]
    #[case(SqlAuthenticationMethod::SqlCertificate, "Sql Certificate")]
    fn test_to_string(#[case] value: SqlAuthenticationMethod, #[case] expected: &str) {
        assert_eq!(expected, value.to_string());
    }
}
