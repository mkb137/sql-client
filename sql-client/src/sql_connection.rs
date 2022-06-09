use crate::sql_connection_string::SqlConnectionString;
use crate::sql_credential::SqlCredential;
use crate::{sql_credential, SqlClientError};

/// A connection to a SQL server.
pub struct SqlConnection {
    /// The connection string with which we were initialized.
    connection_string: String,
    /// The parsed connection string.
    connection_options: SqlConnectionString,
    /// The login credentials
    sql_credential: Option<SqlCredential>,
}
impl SqlConnection {
    /// Tries to create a new connection given a connection string.
    pub fn new(connection_string: &str) -> Result<Self, SqlClientError> {
        // Create the connection string object
        let connection_options: SqlConnectionString = connection_string.try_into()?;
        // Get the credentials from the string
        let sql_credential = connection_options.sql_credential()?;
        // Return the connection
        Ok(Self {
            connection_string: connection_string.to_string(),
            connection_options,
            sql_credential,
        })
    }
    /// Tries to create a new connection given a connection string and credentials.
    pub fn new_auth(
        connection_string: &str,
        sql_credential: SqlCredential,
    ) -> Result<Self, SqlClientError> {
        // Create the connection string object
        let connection_options: SqlConnectionString = connection_string.try_into()?;
        // Return the connection string
        Ok(Self {
            connection_string: connection_string.to_string(),
            connection_options,
            sql_credential: Some(sql_credential),
        })
    }
}
impl Clone for SqlConnection {
    /// Clones the connection.
    fn clone(&self) -> Self {
        // If credentials were supplied originally...
        if let Some(sql_credential) = &self.sql_credential {
            // Create another connection with the same credentials
            SqlConnection::new_auth(
                self.connection_string.clone().as_str(),
                sql_credential.clone(),
            )
            // If we succeeded the first time we should not fail now.
            .unwrap()
        }
        // If credentials were not supplied...
        else {
            // Create another connection from the connection string alone.
            SqlConnection::new(self.connection_string.clone().as_str()).unwrap()
        }
    }
}
