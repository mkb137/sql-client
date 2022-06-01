use crate::db_connection_pool::DbConnectionPool;

/// Internal connection properties and state.
pub(crate) struct DbConnectionInternal {
    /// Whether we permit updating the connection string. ???
    allow_set_connection_string: bool,
    /// Whether the password is hidden???
    hide_password: bool,
    /// The current connection state.
    connection_state: u8,
    /// If a pooled connection, the pool that the connection came from.
    connection_pool: Option<DbConnectionPool>,
    /// True when the connection should no longer be used.
    is_connection_doomed: bool,
    /// True when the connection should not longer be pooled.
    cannot_be_pooled: bool,
    /// When the connection was created.
    create_time: chrono::DateTime<chrono::Utc>,
}
