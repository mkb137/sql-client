//! SQL Server login credentials (username and password).
use crate::sql_client_error::SqlClientError;
use crate::sql_connection::SqlConnection;
use secstr::SecStr;

/// User login credentials.
#[derive(Clone)]
pub struct SqlCredential {
    /// The user's ID.
    user_id: String,
    /// The user's password.
    password: SecStr,
}

impl SqlCredential {
    /// Creates a new SQL credential given a username and password.
    pub fn new(user_id: String, password: SecStr) -> Result<Self, SqlClientError> {
        // If the user ID is too long, return an error.
        if user_id.len() > super::tds_enums::LoginValidationRules::MAXLEN_CLIENTID {
            return Err(SqlClientError::InvalidArgumentLength(
                "user_id".to_string(),
                user_id,
                super::tds_enums::LoginValidationRules::MAXLEN_CLIENTID,
            ));
        }
        // If the password is too long, return an error.
        let password_length = password.unsecure().len();
        if password_length > super::tds_enums::LoginValidationRules::MAXLEN_CLIENTID {
            return Err(SqlClientError::InvalidArgumentLength(
                "password".to_string(),
                "(redacted)".to_string(),
                super::tds_enums::LoginValidationRules::MAXLEN_CLIENTSECRET,
            ));
        }
        // Return a new credential
        Ok(Self { user_id, password })
    }

    /// Returns the user's password.
    pub fn password(&self) -> SecStr {
        self.password.clone()
    }

    /// Returns the user ID.
    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }
}
