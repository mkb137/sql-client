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
    /// An argument was null.
    #[error("A value was not supplied for the argument {0}")]
    ArgumentNull(String),
    /// The length of an argument was valid (i.e. a string was too long).
    #[error("The value '{1}' supplied for argument {0} was greater than length {2}")]
    InvalidArgumentLength(String, String, usize),
}
