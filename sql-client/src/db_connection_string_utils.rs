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
}
