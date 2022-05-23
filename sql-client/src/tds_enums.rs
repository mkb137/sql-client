/// Connection String validation pub constants.
pub(crate) mod login_validation_rules {
    /// Maximum length of the client machine name.
    pub const MAXLEN_HOSTNAME: usize = 128; // the client machine name
    /// Maximum length of the user ID.
    pub const MAXLEN_CLIENTID: usize = 128;
    /// Maximum length of the user's password.
    pub const MAXLEN_CLIENTSECRET: usize = 128;
    /// Maximum length of the application name.
    pub const MAXLEN_APPNAME: usize = 128; // the client application name
    /// Maximum length of the server name.
    pub const MAXLEN_SERVERNAME: usize = 128; // the server name
    /// Maximum length of the interface library.
    pub const MAXLEN_CLIENTINTERFACE: usize = 128; // the interface library name
    /// Maximum length of the language setting.
    pub const MAXLEN_LANGUAGE: usize = 128; // the initial language
    /// Maximum length of the initial database settings.
    pub const MAXLEN_DATABASE: usize = 128; // the initial database
    /// Maximum length of the attach DB file name.
    pub const MAXLEN_ATTACHDBFILE: usize = 260; // the filename for a database that is to be attached during the connection process
}
