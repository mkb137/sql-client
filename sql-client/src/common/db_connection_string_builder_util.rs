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

    #[test]
    fn test_convert_to_boolean() {
        assert_eq!(true, convert_to_boolean("yes").unwrap());
        assert_eq!(true, convert_to_boolean("YES").unwrap());
        assert_eq!(true, convert_to_boolean(" YeS ").unwrap());

        assert_eq!(true, convert_to_boolean("true").unwrap());
        assert_eq!(true, convert_to_boolean("TRUE").unwrap());
        assert_eq!(true, convert_to_boolean(" TrUe ").unwrap());

        assert_eq!(false, convert_to_boolean("no").unwrap());
        assert_eq!(false, convert_to_boolean("NO").unwrap());
        assert_eq!(false, convert_to_boolean(" No ").unwrap());

        assert_eq!(false, convert_to_boolean("false").unwrap());
        assert_eq!(false, convert_to_boolean("FALSE").unwrap());
        assert_eq!(false, convert_to_boolean(" FaLsE").unwrap());
    }

    #[test]
    fn test_convert_to_integrated_security() {
        assert_eq!(true, convert_to_integrated_security("sspi").unwrap());
        assert_eq!(true, convert_to_integrated_security("SSPI").unwrap());
        assert_eq!(true, convert_to_integrated_security(" SsPi ").unwrap());

        assert_eq!(true, convert_to_integrated_security("yes").unwrap());
        assert_eq!(true, convert_to_integrated_security("YES").unwrap());
        assert_eq!(true, convert_to_integrated_security(" YeS ").unwrap());

        assert_eq!(true, convert_to_integrated_security("true").unwrap());
        assert_eq!(true, convert_to_integrated_security("TRUE").unwrap());
        assert_eq!(true, convert_to_integrated_security(" TrUe ").unwrap());

        assert_eq!(false, convert_to_integrated_security("no").unwrap());
        assert_eq!(false, convert_to_integrated_security("NO").unwrap());
        assert_eq!(false, convert_to_integrated_security(" No ").unwrap());

        assert_eq!(false, convert_to_integrated_security("false").unwrap());
        assert_eq!(false, convert_to_integrated_security("FALSE").unwrap());
        assert_eq!(false, convert_to_integrated_security(" FaLsE").unwrap());
    }
}

#[test]
fn test_convert_to_pool_blocking_period() {
    assert_eq!(
        PoolBlockingPeriod::Auto,
        convert_to_pool_blocking_period("auto").unwrap(),
        "auto"
    );
    assert_eq!(
        PoolBlockingPeriod::Auto,
        convert_to_pool_blocking_period("AUTO").unwrap()
    );
    assert_eq!(
        PoolBlockingPeriod::Auto,
        convert_to_pool_blocking_period(" AuTo ").unwrap()
    );

    assert_eq!(
        PoolBlockingPeriod::AlwaysBlock,
        convert_to_pool_blocking_period("alwaysblock").unwrap()
    );
    assert_eq!(
        PoolBlockingPeriod::AlwaysBlock,
        convert_to_pool_blocking_period(" AlwaysBlock ").unwrap()
    );

    assert_eq!(
        PoolBlockingPeriod::NeverBlock,
        convert_to_pool_blocking_period("neverblock").unwrap()
    );
    assert_eq!(
        PoolBlockingPeriod::NeverBlock,
        convert_to_pool_blocking_period(" NeverBlock ").unwrap()
    );
}

#[test]
fn test_convert_to_application_intent() {
    assert_eq!(
        ApplicationIntent::ReadWrite,
        convert_to_application_intent("readwrite").unwrap()
    );
    assert_eq!(
        ApplicationIntent::ReadWrite,
        convert_to_application_intent("ReadWrite").unwrap()
    );
    assert_eq!(
        ApplicationIntent::ReadOnly,
        convert_to_application_intent("readonly").unwrap()
    );
    assert_eq!(
        ApplicationIntent::ReadOnly,
        convert_to_application_intent("ReadOnly").unwrap()
    );
}
