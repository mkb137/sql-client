/// The SqlClient Error type.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqlClientError {
    /// A connection string value was not in form "Name=Value".
    #[error("The connection string parameter '{0}' was not in the expected 'Name=Value' format.")]
    UnsupportedFormat(String),
    /// A connection string value was not in the correct format.
    #[error("Could not convert '{1}' to a {0}")]
    UnsupportedValue(String, String),
    /// The connection string contained an unsupported keyword.
    #[error("The keyword '{0}' is not supported.")]
    UnsupportedKeyword(String),
}
