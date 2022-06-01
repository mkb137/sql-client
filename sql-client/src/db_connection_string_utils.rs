use crate::SqlClientError;

/// Converts a (true/yes,false/no) string to a boolean.
pub(crate) fn convert_to_boolean(value: &str) -> Result<bool, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(SqlClientError::UnsupportedValue(
            "boolean".to_string(),
            value.to_string(),
        )),
    }
}

/// Converts a (true/yes/sspi,false/no) string to a boolean.
pub(crate) fn convert_to_integrated_security(value: &str) -> Result<bool, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" | "sspi" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(SqlClientError::UnsupportedValue(
            "Integrated Security".to_string(),
            value.to_string(),
        )),
    }
}

const LOCAL_DB_PREFIX: &'static str = "(localdb)\\";
const LOCAL_DB_PREFIX_NP: &'static str = "np:\\\\.\\pipe\\LOCALDB#";

//
pub(crate) fn get_local_db_instance_name_from_server_name(server_name: &str) -> Option<String> {
    // If the server starts with the regular prefix...
    if server_name.starts_with(LOCAL_DB_PREFIX) {
        // Pull the name off the end
        let instance_name = &server_name[LOCAL_DB_PREFIX.len()..];
        Some(instance_name.to_string())
    }
    // If, instead, the server name starts with the NP prefix
    else if server_name.starts_with(LOCAL_DB_PREFIX_NP) {
        // Pull the name off the end
        let instance_name = &server_name[LOCAL_DB_PREFIX_NP.len()..];
        Some(instance_name.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApplicationIntent;
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
    #[case("sspi", None)]
    #[case("(localdb)\\SOME_NAME", Some("SOME_NAME"))]
    #[case("np:\\\\.\\pipe\\LOCALDB#ANOTHER_NAME", Some("ANOTHER_NAME"))]
    fn test_get_local_db_instance_name_from_server_name(
        #[case] value: &str,
        #[case] expected: Option<&str>,
    ) {
        let actual = get_local_db_instance_name_from_server_name(value);
        assert_eq!(expected.map(|e| e.to_string()), actual);
    }
}
