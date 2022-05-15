use crate::common::{ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod};
use anyhow::anyhow;

/// Converts a true/false/yes/no string to a boolean.
fn convert_to_boolean(value: &str) -> anyhow::Result<bool> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(anyhow!("Could not convert {:?} to boolean", value)),
    }
}

fn convert_to_integrated_security(value: &str) -> anyhow::Result<bool> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" | "sspi" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(anyhow!(
            "Could not convert {:?} to integrated security",
            value
        )),
    }
}

fn convert_to_pool_blocking_period(value: &str) -> anyhow::Result<PoolBlockingPeriod> {
    match value.trim().to_lowercase().as_str() {
        "auto" => Ok(PoolBlockingPeriod::Auto),
        "alwaysblock" => Ok(PoolBlockingPeriod::AlwaysBlock),
        "neverblock" => Ok(PoolBlockingPeriod::NeverBlock),
        _ => Err(anyhow!(
            "Could not convert {:?} to a pool blocking period",
            value
        )),
    }
}

fn convert_to_application_intent(value: &str) -> anyhow::Result<ApplicationIntent> {
    match value.trim().to_lowercase().as_str() {
        "readonly" => Ok(ApplicationIntent::ReadOnly),
        "readwrite" => Ok(ApplicationIntent::ReadWrite),
        _ => Err(anyhow!(
            "Could not convert {:?} to an application intent",
            value
        )),
    }
}

fn convert_to_authentication_method(value: &str) -> anyhow::Result<SqlAuthenticationMethod> {
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

        _ => Err(anyhow!(
            "Could not convert {:?} to an authentication method",
            value
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest;

    #[rstest::rstest]
    #[case("yes", true)]
    #[case("YES", true)]
    #[case(" YeS ", true)]
    #[case("true", true)]
    #[case("TRUE", true)]
    #[case(" TrUe ", true)]
    #[case("no", false)]
    #[case("NO", false)]
    #[case("No", false)]
    #[case(" No ", false)]
    #[case("false", false)]
    #[case("FALSE", false)]
    #[case("False", false)]
    #[case(" FaLsE ", false)]
    fn test_convert_to_boolean(#[case] value: &str, #[case] expected: bool) {
        match convert_to_boolean(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

    #[rstest::rstest]
    #[case("sspi", true)]
    #[case("SSPI", true)]
    #[case(" SsPi ", true)]
    #[case("yes", true)]
    #[case("YES", true)]
    #[case(" YeS ", true)]
    #[case("true", true)]
    #[case("TRUE", true)]
    #[case(" TrUe ", true)]
    #[case("no", false)]
    #[case("NO", false)]
    #[case("No", false)]
    #[case(" No ", false)]
    #[case("false", false)]
    #[case("FALSE", false)]
    #[case("False", false)]
    #[case(" FaLsE ", false)]
    fn test_convert_to_integrated_security(#[case] value: &str, #[case] expected: bool) {
        match convert_to_integrated_security(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

    #[rstest::rstest]
    #[case("auto", PoolBlockingPeriod::Auto)]
    #[case("AUTO", PoolBlockingPeriod::Auto)]
    #[case(" AuTo ", PoolBlockingPeriod::Auto)]
    #[case("alwaysblock", PoolBlockingPeriod::AlwaysBlock)]
    #[case("AlwaysBlock", PoolBlockingPeriod::AlwaysBlock)]
    #[case(" AlwaysBlock ", PoolBlockingPeriod::AlwaysBlock)]
    #[case("neverblock", PoolBlockingPeriod::NeverBlock)]
    #[case("NeverBlock", PoolBlockingPeriod::NeverBlock)]
    #[case(" NeverBlock ", PoolBlockingPeriod::NeverBlock)]
    fn test_convert_to_pool_blocking_period(
        #[case] value: &str,
        #[case] expected: PoolBlockingPeriod,
    ) {
        match convert_to_pool_blocking_period(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

    #[rstest::rstest]
    #[case("readwrite", ApplicationIntent::ReadWrite)]
    #[case("ReadWrite", ApplicationIntent::ReadWrite)]
    #[case("readonly", ApplicationIntent::ReadOnly)]
    #[case("ReadOnly", ApplicationIntent::ReadOnly)]
    fn test_convert_to_application_intent(
        #[case] value: &str,
        #[case] expected: ApplicationIntent,
    ) {
        match convert_to_application_intent(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

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
    fn test_convert_to_authentication_method(
        #[case] value: &str,
        #[case] expected: SqlAuthenticationMethod,
    ) {
        match convert_to_authentication_method(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }
}
