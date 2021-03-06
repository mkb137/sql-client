use super::db_connection_string_defaults::DbConnectionStringDefaults;
use super::db_connection_string_keywords::{
    DbConnectionStringKeywords, DbConnectionStringKeywordsLower,
};
use super::db_connection_string_utils::*;
use crate::{
    ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod, SqlClientError,
    SqlConnectionColumnEncryptionSetting, SqlConnectionIpAddressPreference,
};
use secstr::SecStr;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

// The available keywords
enum Keyword {
    ApplicationIntent,
    ApplicationName,
    AttachDbFilename,
    Authentication,
    ColumnEncryptionSetting,
    ConnectRetryCount,
    ConnectRetryInterval,
    ConnectTimeout,
    CommandTimeout,
    CurrentLanguage,
    DataSource,
    EnclaveAttestationUrl,
    Encrypt,
    Enlist,
    FailoverPartner,
    InitialCatalog,
    IntegratedSecurity,
    IpAddressPreference,
    LoadBalanceTimeout,
    MaxPoolSize,
    MinPoolSize,
    MultipleActiveResultSets,
    MultiSubnetFailover,
    PacketSize,
    Password,
    PersistSecurityInfo,
    Pooling,
    PoolBlockingPeriod,
    Replication,
    TransactionBinding,
    TypeSystemVersion,
    UserId,
    WorkstationId,
    TrustServerCertificate,
    UserInstance,
}

// Defaults

/// Appends a keyword/value pair to a connection string.
fn append(connection_string: &mut String, keyword: &str, value: &str) {
    // If we have existing values...
    if connection_string.len() > 0 {
        // Add the value delimiter
        connection_string.push(';');
    }
    // Add the keyword
    connection_string.push_str(keyword);
    // Add the delimiter
    connection_string.push('=');
    // Add the value
    connection_string.push_str(value);
}

/// Appends a keyword/value pair to a connection string.
fn append_str(connection_string: &mut String, keyword: &str, value: String) {
    append(connection_string, keyword, value.as_str());
}

/// Appends a keyword/value pair to a connection string, formatting the bool correctly.
fn append_bool(connection_string: &mut String, keyword: &str, value: &bool) {
    append(
        connection_string,
        keyword,
        if *value { "True" } else { "False" },
    );
}

/// Appends a keyword/value pair to a connection string if the value is not "None".
fn append_opt(connection_string: &mut String, keyword: &str, value: Option<String>) {
    if let Some(v) = value {
        append_str(connection_string, keyword, v);
    }
}

/// A utility struct for creating well-formatted and valid SQL connection strings.
///
/// # Examples
///
/// ```
/// use sql_client::SqlConnectionStringBuilder;
/// let mut builder = SqlConnectionStringBuilder::default();
/// builder.set_data_source(Some("server_name".to_string()));
/// builder.set_initial_catalog(Some("MyDb".to_string()));
/// assert_eq!("Data Source=server_name;Initial Catalog=MyDb".to_string(), builder.connection_string());
/// ```
pub struct SqlConnectionStringBuilder {
    /// Declares the application workload type when connecting to a database in an SQL Server Availability Group.
    application_intent: ApplicationIntent,
    /// The name of the application associated with the connection string.
    application_name: String,
    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    attach_db_filename: Option<String>,
    /// ?
    authentication: SqlAuthenticationMethod,
    /// ?
    column_encryption_setting: SqlConnectionColumnEncryptionSetting,
    /// The number of reconnections attempted after identifying that there was an idle connection failure. This must be an integer between 0 and 255. Default is 1. Set to 0 to disable reconnecting on idle connection failures.
    connect_retry_count: u8,
    /// Amount of time (in seconds) between each reconnection attempt after identifying that there was an idle connection failure. This must be an integer between 1 and 60. The default is 10 seconds.
    connect_retry_interval: u8,
    /// The length of time (in seconds) to wait for a connection to the server before terminating the attempt and generating an error.
    connect_timeout: u16,
    /// The length of time (in seconds) to wait for a command to the server before terminating the attempt and generating an error.
    command_timeout: u16,
    /// The SQL Server Language record name.
    current_language: Option<String>,
    /// The name or network address of the instance of SQL Server to connect to.
    data_source: Option<String>,
    /// ?
    enclave_attestation_url: Option<String>,
    /// Whether SQL Server uses SSL encryption for all data sent between the client and server if the server has a certificate installed.
    encrypt: bool,
    /// Whether the SQL Server connection pooler automatically enlists the connection in the creation thread's current transaction context.
    enlist: bool,
    /// The name or address of the partner server to connect to if the primary server is down.
    failover_partner: Option<String>,
    /// The name of the database associated with the connection.
    initial_catalog: Option<String>,
    /// Whether User ID and Password are specified in the connection (when false) or whether the current Windows account credentials are used for authentication (when true).
    integrated_security: bool,
    /// ?
    ip_address_preference: SqlConnectionIpAddressPreference,
    /// ??
    load_balance_timeout: u16,
    /// The maximum number of connections allowed in the connection pool for this specific connection string.
    max_pool_size: u8,
    /// The minimum number of connections allowed in the connection pool for this specific connection string.
    min_pool_size: u8,
    /// When true, an application can maintain multiple active result sets (MARS). When false, an application must process or cancel all result sets from one batch before it can execute any other batch on that connection.
    multiple_active_result_sets: bool,
    /// If your application is connecting to an Always On availability group (AG) or Always On Failover Cluster Instance (FCI) on different subnets, setting MultiSubnetFailover=true provides faster detection of and connection to the (currently) active server.
    multi_subnet_failover: bool,
    /// The size in bytes of the network packets used to communicate with an instance of SQL Server.
    packet_size: u16,
    /// The password for the SQL Server account.
    password: Option<SecStr>,
    /// Indicates if security-sensitive information, such as the password or access token, should be returned as part of the connection string on a connection created with this SqlConnectionStringBuilder after that connection has ever been in an open state.
    persist_security_info: bool,
    /// Whether the connection will be pooled or explicitly opened every time that the connection is requested.
    pooling: bool,
    /// The blocking period behavior for a connection pool.
    pool_blocking_period: PoolBlockingPeriod,
    /// Whether replication is supported using the connection.
    replication: bool,
    /// Indicates how the connection maintains its association with an enlisted System.Transactions transaction.
    transaction_binding: String,
    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    trust_server_certificate: bool,
    /// Indicates the type system the application expects.
    type_system_version: String,
    /// The user ID to be used when connecting to SQL Server.
    user_id: Option<String>,
    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    user_instance: bool,
    /// The name of the workstation connecting to SQL Server.
    workstation_id: Option<String>,
    /// The keywords that have been set.  Used to produce a connection string only with the values that have been set explicitly.
    keywords_in_use: Vec<Keyword>,
}

impl SqlConnectionStringBuilder {
    /// Declares the application workload type when connecting to a database in an SQL Server Availability Group.
    pub fn application_intent(&self) -> ApplicationIntent {
        self.application_intent.clone()
    }

    /// The name of the application associated with the connection string.
    pub fn application_name(&self) -> String {
        self.application_name.clone()
    }

    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    pub fn attach_db_filename(&self) -> Option<String> {
        self.attach_db_filename.clone()
    }

    /// ?
    pub fn authentication(&self) -> SqlAuthenticationMethod {
        self.authentication.clone()
    }

    /// ?
    pub fn column_encryption_setting(&self) -> SqlConnectionColumnEncryptionSetting {
        self.column_encryption_setting.clone()
    }

    /// The number of reconnections attempted after identifying that there was an idle connection failure. This must be an integer between 0 and 255. Default is 1. Set to 0 to disable reconnecting on idle connection failures.
    pub fn connect_retry_count(&self) -> u8 {
        self.connect_retry_count.clone()
    }

    /// Amount of time (in seconds) between each reconnection attempt after identifying that there was an idle connection failure. This must be an integer between 1 and 60. The default is 10 seconds.
    pub fn connect_retry_interval(&self) -> u8 {
        self.connect_retry_interval.clone()
    }

    /// The length of time (in seconds) to wait for a connection to the server before terminating the attempt and generating an error.
    pub fn connect_timeout(&self) -> u16 {
        self.connect_timeout.clone()
    }

    /// The length of time (in seconds) to wait for a command to the server before terminating the attempt and generating an error.
    pub fn command_timeout(&self) -> u16 {
        self.command_timeout.clone()
    }

    /// The SQL Server Language record name.
    pub fn current_language(&self) -> Option<String> {
        self.current_language.clone()
    }

    /// The name or network address of the instance of SQL Server to connect to.
    pub fn data_source(&self) -> Option<String> {
        self.data_source.clone()
    }

    /// ?
    pub fn enclave_attestation_url(&self) -> Option<String> {
        self.enclave_attestation_url.clone()
    }

    /// Whether SQL Server uses SSL encryption for all data sent between the client and server if the server has a certificate installed.
    pub fn encrypt(&self) -> bool {
        self.encrypt.clone()
    }

    /// Whether the SQL Server connection pooler automatically enlists the connection in the creation thread's current transaction context.
    pub fn enlist(&self) -> bool {
        self.enlist.clone()
    }

    /// The name or address of the partner server to connect to if the primary server is down.
    pub fn failover_partner(&self) -> Option<String> {
        self.failover_partner.clone()
    }

    /// The name of the database associated with the connection.
    pub fn initial_catalog(&self) -> Option<String> {
        self.initial_catalog.clone()
    }

    /// Whether User ID and Password are specified in the connection (when false) or whether the current Windows account credentials are used for authentication (when true).
    pub fn integrated_security(&self) -> bool {
        self.integrated_security.clone()
    }

    /// ?
    pub fn ip_address_preference(&self) -> SqlConnectionIpAddressPreference {
        self.ip_address_preference.clone()
    }
    /// ??
    pub fn load_balance_timeout(&self) -> u16 {
        self.load_balance_timeout.clone()
    }

    /// The maximum number of connections allowed in the connection pool for this specific connection string.
    pub fn max_pool_size(&self) -> u8 {
        self.max_pool_size.clone()
    }

    /// The minimum number of connections allowed in the connection pool for this specific connection string.
    pub fn min_pool_size(&self) -> u8 {
        self.min_pool_size.clone()
    }

    /// When true{ self.x.clone() } an application can maintain multiple active result sets (MARS). When false{ self.x.clone() } an application must process or cancel all result sets from one batch before it can execute any other batch on that connection.
    pub fn multiple_active_result_sets(&self) -> bool {
        self.multiple_active_result_sets.clone()
    }

    /// If your application is connecting to an Always On availability group (AG) or Always On Failover Cluster Instance (FCI) on different subnets{ self.x.clone() } setting MultiSubnetFailover=true provides faster detection of and connection to the (currently) active server.
    pub fn multi_subnet_failover(&self) -> bool {
        self.multi_subnet_failover.clone()
    }

    /// The size in bytes of the network packets used to communicate with an instance of SQL Server.
    pub fn packet_size(&self) -> u16 {
        self.packet_size.clone()
    }

    /// The password for the SQL Server account.
    pub fn password(&self) -> Option<SecStr> {
        self.password.clone()
    }

    /// Indicates if security-sensitive information{ self.x.clone() } such as the password or access token{ self.x.clone() } should be returned as part of the connection string on a connection created with this SqlConnectionStringBuilder after that connection has ever been in an open state.
    pub fn persist_security_info(&self) -> bool {
        self.persist_security_info.clone()
    }

    /// Whether the connection will be pooled or explicitly opened every time that the connection is requested.
    pub fn pooling(&self) -> bool {
        self.pooling.clone()
    }

    /// The blocking period behavior for a connection pool.
    pub fn pool_blocking_period(&self) -> PoolBlockingPeriod {
        self.pool_blocking_period.clone()
    }

    /// Whether replication is supported using the connection.
    pub fn replication(&self) -> bool {
        self.replication.clone()
    }

    /// Indicates how the connection maintains its association with an enlisted System.Transactions transaction.
    pub fn transaction_binding(&self) -> String {
        self.transaction_binding.clone()
    }

    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    pub fn trust_server_certificate(&self) -> bool {
        self.trust_server_certificate.clone()
    }

    /// Indicates the type system the application expects.
    pub fn type_system_version(&self) -> String {
        self.type_system_version.clone()
    }

    /// The user ID to be used when connecting to SQL Server.
    pub fn user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    pub fn user_instance(&self) -> bool {
        self.user_instance.clone()
    }

    /// The name of the workstation connecting to SQL Server.
    pub fn workstation_id(&self) -> Option<String> {
        self.workstation_id.clone()
    }

    /// Returns the connection string.
    pub fn connection_string(&self) -> String {
        // Start with a blank connection string
        let mut value = String::new();
        // For each of the keywords that were overridden by the user...
        for keyword in &self.keywords_in_use {
            match keyword {
                Keyword::ApplicationIntent => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::APPLICATION_INTENT,
                        self.application_intent.to_string(),
                    );
                }
                Keyword::ApplicationName => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::APPLICATION_NAME,
                        self.application_name.clone(),
                    );
                }
                Keyword::AttachDbFilename => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::ATTACH_DB_FILENAME,
                        self.attach_db_filename.clone(),
                    );
                }
                Keyword::Authentication => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::AUTHENTICATION,
                        self.authentication.to_string(),
                    );
                }
                Keyword::ColumnEncryptionSetting => append_str(
                    &mut value,
                    DbConnectionStringKeywords::COLUMN_ENCRYPTION_SETTING,
                    self.column_encryption_setting.to_string(),
                ),
                Keyword::ConnectRetryCount => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::CONNECT_RETRY_COUNT,
                        self.connect_retry_count.to_string(),
                    );
                }
                Keyword::ConnectRetryInterval => append_str(
                    &mut value,
                    DbConnectionStringKeywords::CONNECT_RETRY_INTERVAL,
                    self.connect_retry_interval.to_string(),
                ),
                Keyword::ConnectTimeout => append_str(
                    &mut value,
                    DbConnectionStringKeywords::CONNECT_TIMEOUT,
                    self.connect_timeout.to_string(),
                ),
                Keyword::CommandTimeout => append_str(
                    &mut value,
                    DbConnectionStringKeywords::COMMAND_TIMEOUT,
                    self.command_timeout.to_string(),
                ),
                Keyword::CurrentLanguage => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::CURRENT_LANGUAGE,
                        self.current_language.clone(),
                    );
                }
                Keyword::DataSource => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::DATA_SOURCE,
                        self.data_source.clone(),
                    );
                }
                Keyword::EnclaveAttestationUrl => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::ENCLAVE_ATTESTATION_URL,
                        self.enclave_attestation_url.clone(),
                    );
                }
                Keyword::Encrypt => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::ENCRYPT,
                        &self.encrypt,
                    );
                }
                Keyword::Enlist => {
                    append_bool(&mut value, DbConnectionStringKeywords::ENLIST, &self.enlist);
                }
                Keyword::FailoverPartner => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::FAILOVER_PARTNER,
                        self.failover_partner.clone(),
                    );
                }
                Keyword::InitialCatalog => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::INITIAL_CATALOG,
                        self.initial_catalog.clone(),
                    );
                }
                Keyword::IntegratedSecurity => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::INTEGRATED_SECURITY,
                        &self.integrated_security,
                    );
                }
                Keyword::IpAddressPreference => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::IP_ADDRESS_PREFERENCE,
                        self.ip_address_preference.to_string(),
                    );
                }
                Keyword::LoadBalanceTimeout => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::LOAD_BALANCE_TIMEOUT,
                        self.load_balance_timeout.to_string(),
                    );
                }
                Keyword::MaxPoolSize => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::MAX_POOL_SIZE,
                        self.max_pool_size.to_string(),
                    );
                }
                Keyword::MinPoolSize => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::MIN_POOL_SIZE,
                        self.min_pool_size.to_string(),
                    );
                }
                Keyword::MultipleActiveResultSets => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::MULTIPLE_ACTIVE_RESULT_SETS,
                        &self.multiple_active_result_sets,
                    );
                }
                Keyword::MultiSubnetFailover => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::MULTI_SUBNET_FAILOVER,
                        &self.multi_subnet_failover,
                    );
                }
                Keyword::PacketSize => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::PACKET_SIZE,
                        self.packet_size.to_string(),
                    );
                }
                Keyword::Password => {
                    let pwd = self
                        .password
                        .clone()
                        .map(|pwd| String::from_utf8_lossy(pwd.unsecure()).to_string());
                    append_opt(&mut value, DbConnectionStringKeywords::PASSWORD, pwd);
                }
                Keyword::PersistSecurityInfo => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::PERSIST_SECURITY_INFO,
                        &self.persist_security_info,
                    );
                }
                Keyword::Pooling => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::POOLING,
                        &self.pooling,
                    );
                }
                Keyword::PoolBlockingPeriod => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::POOL_BLOCKING_PERIOD,
                        self.pool_blocking_period.to_string(),
                    );
                }
                Keyword::Replication => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::REPLICATION,
                        &self.replication,
                    );
                }
                Keyword::TransactionBinding => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::TRANSACTION_BINDING,
                        self.transaction_binding.to_string(),
                    );
                }
                Keyword::TypeSystemVersion => {
                    append_str(
                        &mut value,
                        DbConnectionStringKeywords::TYPE_SYSTEM_VERSION,
                        self.type_system_version.to_string(),
                    );
                }
                Keyword::UserId => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::USER_ID,
                        self.user_id.clone(),
                    );
                }
                Keyword::WorkstationId => {
                    append_opt(
                        &mut value,
                        DbConnectionStringKeywords::WORKSTATION_ID,
                        self.workstation_id.clone(),
                    );
                }
                Keyword::TrustServerCertificate => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::TRUST_SERVER_CERTIFICATE,
                        &self.trust_server_certificate,
                    );
                }
                Keyword::UserInstance => {
                    append_bool(
                        &mut value,
                        DbConnectionStringKeywords::USER_INSTANCE,
                        &self.user_instance,
                    );
                }
            }
        }
        // Return the value
        value
    }

    /// Declares the application workload type when connecting to a database in an SQL Server Availability Group.
    pub fn set_application_intent(&mut self, value: ApplicationIntent) {
        self.application_intent = value;
        self.keywords_in_use.push(Keyword::ApplicationIntent);
    }

    /// The name of the application associated with the connection string.
    pub fn set_application_name(&mut self, value: String) {
        self.application_name = value;
        self.keywords_in_use.push(Keyword::ApplicationName);
    }

    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    pub fn set_attach_db_filename(&mut self, value: Option<String>) {
        self.attach_db_filename = value;
        self.keywords_in_use.push(Keyword::AttachDbFilename);
    }

    /// ?
    pub fn set_authentication(&mut self, value: SqlAuthenticationMethod) {
        self.authentication = value;
        self.keywords_in_use.push(Keyword::Authentication);
    }

    /// ?
    pub fn set_column_encryption_setting(&mut self, value: SqlConnectionColumnEncryptionSetting) {
        self.column_encryption_setting = value;
        self.keywords_in_use.push(Keyword::ColumnEncryptionSetting);
    }

    /// The length of time (in seconds) to wait for a command to the server before terminating the attempt and generating an error.
    pub fn set_command_timeout(&mut self, value: u16) {
        self.command_timeout = value;
        self.keywords_in_use.push(Keyword::CommandTimeout);
    }

    /// The number of reconnections attempted after identifying that there was an idle connection failure. This must be an integer between 0 and 255. Default is 1. Set to 0 to disable reconnecting on idle connection failures.
    pub fn set_connect_retry_count(&mut self, value: u8) {
        self.connect_retry_count = value;
        self.keywords_in_use.push(Keyword::ConnectRetryCount);
    }

    /// Amount of time (in seconds) between each reconnection attempt after identifying that there was an idle connection failure. This must be an integer between 1 and 60. The default is 10 seconds.
    pub fn set_connect_retry_interval(&mut self, value: u8) {
        self.connect_retry_interval = value;
        self.keywords_in_use.push(Keyword::ConnectRetryInterval);
    }

    /// The length of time (in seconds) to wait for a connection to the server before terminating the attempt and generating an error.
    pub fn set_connect_timeout(&mut self, value: u16) {
        self.connect_timeout = value;
        self.keywords_in_use.push(Keyword::ConnectTimeout);
    }

    /// The SQL Server Language record name.
    pub fn set_current_language(&mut self, value: Option<String>) {
        self.current_language = value;
        self.keywords_in_use.push(Keyword::CurrentLanguage);
    }

    /// The name or network address of the instance of SQL Server to connect to.
    pub fn set_data_source(&mut self, value: Option<String>) {
        self.data_source = value;
        self.keywords_in_use.push(Keyword::DataSource);
    }

    /// ?
    pub fn set_enclave_attestation_url(&mut self, value: Option<String>) {
        self.enclave_attestation_url = value;
        self.keywords_in_use.push(Keyword::EnclaveAttestationUrl);
    }

    /// Whether SQL Server uses SSL encryption for all data sent between the client and server if the server has a certificate installed.
    pub fn set_encrypt(&mut self, value: bool) {
        self.encrypt = value;
        self.keywords_in_use.push(Keyword::Encrypt);
    }

    /// Whether the SQL Server connection pooler automatically enlists the connection in the creation thread's current transaction context.
    pub fn set_enlist(&mut self, value: bool) {
        self.enlist = value;
        self.keywords_in_use.push(Keyword::Enlist);
    }

    /// The name or address of the partner server to connect to if the primary server is down.
    pub fn set_failover_partner(&mut self, value: Option<String>) {
        self.failover_partner = value;
        self.keywords_in_use.push(Keyword::FailoverPartner);
    }

    /// The name of the database associated with the connection.
    pub fn set_initial_catalog(&mut self, value: Option<String>) {
        self.initial_catalog = value;
        self.keywords_in_use.push(Keyword::InitialCatalog);
    }

    /// Whether User ID and Password are specified in the connection (when false) or whether the current Windows account credentials are used for authentication (when true).
    pub fn set_integrated_security(&mut self, value: bool) {
        self.integrated_security = value;
        self.keywords_in_use.push(Keyword::IntegratedSecurity);
    }

    /// ?
    pub fn set_ip_address_preference(&mut self, value: SqlConnectionIpAddressPreference) {
        self.ip_address_preference = value;
        self.keywords_in_use.push(Keyword::IpAddressPreference);
    }

    /// ??
    pub fn set_load_balance_timeout(&mut self, value: u16) {
        self.load_balance_timeout = value;
        self.keywords_in_use.push(Keyword::LoadBalanceTimeout);
    }

    /// The maximum number of connections allowed in the connection pool for this specific connection string.
    pub fn set_max_pool_size(&mut self, value: u8) {
        self.max_pool_size = value;
        self.keywords_in_use.push(Keyword::MaxPoolSize);
    }

    /// The minimum number of connections allowed in the connection pool for this specific connection string.
    pub fn set_min_pool_size(&mut self, value: u8) {
        self.min_pool_size = value;
        self.keywords_in_use.push(Keyword::MinPoolSize);
    }

    /// When true, an application can maintain multiple active result sets (MARS). When false, an application must process or cancel all result sets from one batch before it can execute any other batch on that connection.
    pub fn set_multiple_active_result_sets(&mut self, value: bool) {
        self.multiple_active_result_sets = value;
        self.keywords_in_use.push(Keyword::MultipleActiveResultSets);
    }

    /// If your application is connecting to an Always On availability group (AG) or Always On Failover Cluster Instance (FCI) on different subnets, setting MultiSubnetFailover=true provides faster detection of and connection to the (currently) active server.
    pub fn set_multi_subnet_failover(&mut self, value: bool) {
        self.multi_subnet_failover = value;
        self.keywords_in_use.push(Keyword::MultiSubnetFailover);
    }

    /// The size in bytes of the network packets used to communicate with an instance of SQL Server.
    pub fn set_packet_size(&mut self, value: u16) {
        self.packet_size = value;
        self.keywords_in_use.push(Keyword::PacketSize);
    }

    /// The password for the SQL Server account.
    pub fn set_password(&mut self, value: Option<SecStr>) {
        self.password = value;
        self.keywords_in_use.push(Keyword::Password);
    }

    /// Indicates if security-sensitive information, such as the password or access token, should be returned as part of the connection string on a connection created with this SqlConnectionStringBuilder after that connection has ever been in an open state.
    pub fn set_persist_security_info(&mut self, value: bool) {
        self.persist_security_info = value;
        self.keywords_in_use.push(Keyword::PersistSecurityInfo);
    }

    /// Whether the connection will be pooled or explicitly opened every time that the connection is requested.
    pub fn set_pooling(&mut self, value: bool) {
        self.pooling = value;
        self.keywords_in_use.push(Keyword::Pooling);
    }

    /// The blocking period behavior for a connection pool.
    pub fn set_pool_blocking_period(&mut self, value: PoolBlockingPeriod) {
        self.pool_blocking_period = value;
        self.keywords_in_use.push(Keyword::PoolBlockingPeriod);
    }

    /// Whether replication is supported using the connection.
    pub fn set_replication(&mut self, value: bool) {
        self.replication = value;
        self.keywords_in_use.push(Keyword::Replication);
    }

    /// Indicates how the connection maintains its association with an enlisted System.Transactions transaction.
    pub fn set_transaction_binding(&mut self, value: String) {
        self.transaction_binding = value;
        self.keywords_in_use.push(Keyword::TransactionBinding);
    }

    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    pub fn set_trust_server_certificate(&mut self, value: bool) {
        self.trust_server_certificate = value;
        self.keywords_in_use.push(Keyword::TrustServerCertificate);
    }

    /// Indicates the type system the application expects.
    pub fn set_type_system_version(&mut self, value: String) {
        self.type_system_version = value;
        self.keywords_in_use.push(Keyword::TypeSystemVersion);
    }

    /// The user ID to be used when connecting to SQL Server.
    pub fn set_user_id(&mut self, value: Option<String>) {
        self.user_id = value;
        self.keywords_in_use.push(Keyword::UserId);
    }

    /// The name of the workstation connecting to SQL Server.
    pub fn set_workstation_id(&mut self, value: Option<String>) {
        self.workstation_id = value;
        self.keywords_in_use.push(Keyword::WorkstationId);
    }

    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    pub fn set_user_instance(&mut self, value: bool) {
        self.user_instance = value;
        self.keywords_in_use.push(Keyword::UserInstance);
    }
}

impl Default for SqlConnectionStringBuilder {
    fn default() -> Self {
        Self {
            application_intent: DbConnectionStringDefaults::APPLICATION_INTENT,
            application_name: DbConnectionStringDefaults::APPLICATION_NAME.to_string(),
            attach_db_filename: DbConnectionStringDefaults::ATTACH_DB_FILENAME,
            authentication: DbConnectionStringDefaults::AUTHENTICATION,
            column_encryption_setting: DbConnectionStringDefaults::COLUMN_ENCRYPTION_SETTING,
            command_timeout: DbConnectionStringDefaults::COMMAND_TIMEOUT,
            connect_retry_count: DbConnectionStringDefaults::CONNECT_RETRY_COUNT,
            connect_retry_interval: DbConnectionStringDefaults::CONNECT_RETRY_INTERVAL,
            connect_timeout: DbConnectionStringDefaults::CONNECT_TIMEOUT,
            current_language: DbConnectionStringDefaults::CURRENT_LANGUAGE,
            data_source: DbConnectionStringDefaults::DATA_SOURCE,
            enclave_attestation_url: DbConnectionStringDefaults::ENCLAVE_ATTESTATION_URL,
            encrypt: DbConnectionStringDefaults::ENCRYPT,
            enlist: DbConnectionStringDefaults::ENLIST,
            failover_partner: DbConnectionStringDefaults::FAILOVER_PARTNER,
            initial_catalog: DbConnectionStringDefaults::INITIAL_CATALOG,
            integrated_security: DbConnectionStringDefaults::INTEGRATED_SECURITY,
            ip_address_preference: DbConnectionStringDefaults::IP_ADDRESS_PREFERENCE,
            load_balance_timeout: DbConnectionStringDefaults::LOAD_BALANCE_TIMEOUT,
            max_pool_size: DbConnectionStringDefaults::MAX_POOL_SIZE,
            min_pool_size: DbConnectionStringDefaults::MIN_POOL_SIZE,
            multiple_active_result_sets: DbConnectionStringDefaults::MULTIPLE_ACTIVE_RESULT_SETS,
            multi_subnet_failover: DbConnectionStringDefaults::MULTI_SUBNET_FAILOVER,
            packet_size: DbConnectionStringDefaults::PACKET_SIZE,
            password: DbConnectionStringDefaults::PASSWORD,
            persist_security_info: DbConnectionStringDefaults::PERSIST_SECURITY_INFO,
            pooling: DbConnectionStringDefaults::POOLING,
            pool_blocking_period: DbConnectionStringDefaults::POOL_BLOCKING_PERIOD,
            replication: DbConnectionStringDefaults::REPLICATION,
            transaction_binding: DbConnectionStringDefaults::TRANSACTION_BINDING_STR.to_string(),
            type_system_version: DbConnectionStringDefaults::TYPE_SYSTEM_VERSION.to_string(),
            user_id: DbConnectionStringDefaults::USER_ID,
            workstation_id: DbConnectionStringDefaults::WORKSTATION_ID,
            trust_server_certificate: DbConnectionStringDefaults::TRUST_SERVER_CERTIFICATE,
            user_instance: DbConnectionStringDefaults::USER_INSTANCE,
            keywords_in_use: Vec::new(),
        }
    }
}

impl TryFrom<&str> for SqlConnectionStringBuilder {
    type Error = SqlClientError;

    /// Parses a connection string into a connection string builder.
    fn try_from(connection_string: &str) -> Result<Self, Self::Error> {
        // Create the default connection string builder
        let mut connection_string_builder = SqlConnectionStringBuilder::default();
        // Connection strings are of the format "NameA = ValueA;NameB = ValueB".
        // Split the connection string by ";" to separate the name/value pairs.
        for key_value_pair in connection_string.split(";") {
            // Try to split the key/value pair into key and value
            if let Some((key, value)) = key_value_pair.split_once("=") {
                // Get the keyword, trimmed and in lowercase so we can better match it.
                let trimmed_key = key.trim();
                let lowercase_key = trimmed_key.to_lowercase();
                let keyword = lowercase_key.as_str();
                // Get the value, trimmed.
                let value = value.trim();
                // Check the keyword against the keywords we know
                match keyword {
                    DbConnectionStringKeywordsLower::APPLICATION_INTENT
                    | DbConnectionStringKeywordsLower::APPLICATION_INTENT_ALT => {
                        let application_intent: ApplicationIntent = value.try_into()?;
                        connection_string_builder.set_application_intent(application_intent);
                    }
                    DbConnectionStringKeywordsLower::APPLICATION_NAME
                    | DbConnectionStringKeywordsLower::APPLICATION_NAME_ALT => {
                        connection_string_builder.set_application_name(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::ATTACH_DB_FILENAME
                    | DbConnectionStringKeywordsLower::ATTACH_DB_FILENAME_ALT => {
                        connection_string_builder.set_attach_db_filename(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::AUTHENTICATION => {
                        let authentication: SqlAuthenticationMethod = value.try_into()?;
                        connection_string_builder.set_authentication(authentication);
                    }
                    DbConnectionStringKeywordsLower::COLUMN_ENCRYPTION_SETTING => {
                        let column_encrpytion_setting: SqlConnectionColumnEncryptionSetting =
                            value.try_into()?;
                        connection_string_builder
                            .set_column_encryption_setting(column_encrpytion_setting);
                    }
                    DbConnectionStringKeywordsLower::COMMAND_TIMEOUT => {
                        let command_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_command_timeout(command_timeout);
                    }
                    DbConnectionStringKeywordsLower::CONNECT_RETRY_COUNT
                    | DbConnectionStringKeywordsLower::CONNECT_RETRY_COUNT_ALT => {
                        let connect_retry_count: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_connect_retry_count(connect_retry_count);
                    }
                    DbConnectionStringKeywordsLower::CONNECT_RETRY_INTERVAL
                    | DbConnectionStringKeywordsLower::CONNECT_RETRY_INTERVAL_ALT => {
                        let connect_retry_interval: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                        connection_string_builder
                            .set_connect_retry_interval(connect_retry_interval);
                    }
                    DbConnectionStringKeywordsLower::CONNECT_TIMEOUT
                    | DbConnectionStringKeywordsLower::CONNECT_TIMEOUT_ALT1
                    | DbConnectionStringKeywordsLower::CONNECT_TIMEOUT_ALT2 => {
                        let connect_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_connect_timeout(connect_timeout);
                    }
                    DbConnectionStringKeywordsLower::CURRENT_LANGUAGE
                    | DbConnectionStringKeywordsLower::CURRENT_LANGUAGE_ALT => {
                        connection_string_builder.set_current_language(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::DATA_SOURCE
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT1
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT2
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT3
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT4 => {
                        connection_string_builder.set_data_source(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::ENCLAVE_ATTESTATION_URL => {
                        connection_string_builder
                            .set_enclave_attestation_url(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::ENCRYPT => {
                        let encrypt = convert_to_boolean(value)?;
                        connection_string_builder.set_encrypt(encrypt);
                    }
                    DbConnectionStringKeywordsLower::ENLIST => {
                        let enlist = convert_to_boolean(value)?;
                        connection_string_builder.set_enlist(enlist);
                    }
                    DbConnectionStringKeywordsLower::FAILOVER_PARTNER => {
                        connection_string_builder.set_failover_partner(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::INITIAL_CATALOG
                    | DbConnectionStringKeywordsLower::INITIAL_CATALOG_ALT => {
                        connection_string_builder.set_initial_catalog(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::INTEGRATED_SECURITY
                    | DbConnectionStringKeywordsLower::INTEGRATED_SECURITY_ALT => {
                        let integrated_security = convert_to_integrated_security(value)?;
                        connection_string_builder.set_integrated_security(integrated_security);
                    }
                    DbConnectionStringKeywordsLower::IP_ADDRESS_PREFERENCE
                    | DbConnectionStringKeywordsLower::IP_ADDRESS_PREFERENCE_ALT => {
                        let ip_address_preference: SqlConnectionIpAddressPreference =
                            value.try_into()?;
                        connection_string_builder.set_ip_address_preference(ip_address_preference);
                    }
                    DbConnectionStringKeywordsLower::LOAD_BALANCE_TIMEOUT
                    | DbConnectionStringKeywordsLower::LOAD_BALANCE_TIMEOUT_ALT => {
                        let load_balance_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_load_balance_timeout(load_balance_timeout);
                    }
                    DbConnectionStringKeywordsLower::MAX_POOL_SIZE => {
                        let max_pool_size: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_max_pool_size(max_pool_size);
                    }
                    DbConnectionStringKeywordsLower::MIN_POOL_SIZE => {
                        let min_pool_size: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_min_pool_size(min_pool_size);
                    }
                    DbConnectionStringKeywordsLower::MULTIPLE_ACTIVE_RESULT_SETS
                    | DbConnectionStringKeywordsLower::MULTIPLE_ACTIVE_RESULT_SETS_ALT => {
                        let multiple_active_result_sets = convert_to_boolean(value)?;
                        connection_string_builder
                            .set_multiple_active_result_sets(multiple_active_result_sets);
                    }
                    DbConnectionStringKeywordsLower::MULTI_SUBNET_FAILOVER
                    | DbConnectionStringKeywordsLower::MULTI_SUBNET_FAILOVER_ALT => {
                        let multi_subnet_failover = convert_to_boolean(value)?;
                        connection_string_builder.set_multi_subnet_failover(multi_subnet_failover);
                    }
                    DbConnectionStringKeywordsLower::PACKET_SIZE => {
                        let packet_size: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_packet_size(packet_size);
                    }
                    DbConnectionStringKeywordsLower::PASSWORD
                    | DbConnectionStringKeywordsLower::PASSWORD_ALT => {
                        connection_string_builder.set_password(Some(SecStr::from(value)));
                    }
                    DbConnectionStringKeywordsLower::PERSIST_SECURITY_INFO
                    | DbConnectionStringKeywordsLower::PERSIST_SECURITY_INFO_ALT => {
                        let persist_security_info = convert_to_boolean(value)?;
                        connection_string_builder.set_persist_security_info(persist_security_info);
                    }
                    DbConnectionStringKeywordsLower::POOLING => {
                        let pooling = convert_to_boolean(value)?;
                        connection_string_builder.set_pooling(pooling);
                    }
                    DbConnectionStringKeywordsLower::POOL_BLOCKING_PERIOD
                    | DbConnectionStringKeywordsLower::POOL_BLOCKING_PERIOD_ALT => {
                        let pool_blocking_period: PoolBlockingPeriod = value.try_into()?;
                        connection_string_builder.set_pool_blocking_period(pool_blocking_period);
                    }
                    DbConnectionStringKeywordsLower::REPLICATION => {
                        let replication = convert_to_boolean(value)?;
                        connection_string_builder.set_replication(replication);
                    }
                    DbConnectionStringKeywordsLower::TRANSACTION_BINDING => {
                        connection_string_builder.set_transaction_binding(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::TRUST_SERVER_CERTIFICATE
                    | DbConnectionStringKeywordsLower::TRUST_SERVER_CERTIFICATE_ALT => {
                        let trust_server_certificate = convert_to_boolean(value)?;
                        connection_string_builder
                            .set_trust_server_certificate(trust_server_certificate);
                    }
                    DbConnectionStringKeywordsLower::TYPE_SYSTEM_VERSION => {
                        connection_string_builder.set_type_system_version(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::USER_ID
                    | DbConnectionStringKeywordsLower::USER_ID_ALT1
                    | DbConnectionStringKeywordsLower::USER_ID_ALT2 => {
                        connection_string_builder.set_user_id(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::WORKSTATION_ID
                    | DbConnectionStringKeywordsLower::WORKSTATION_ALT_ID => {
                        connection_string_builder.set_workstation_id(Some(value.to_string()));
                    }
                    DbConnectionStringKeywordsLower::USER_INSTANCE => {
                        let user_instance = convert_to_boolean(value)?;
                        connection_string_builder.set_user_instance(user_instance);
                    }
                    // If the keyword is not known, return an error.
                    unknown_keyword => {
                        return Err(SqlClientError::UnsupportedKeyword(
                            unknown_keyword.to_string(),
                        ))
                    }
                }
            } else {
                // If the key/value pair couldn't be split, throw an error.
                return Err(SqlClientError::UnsupportedFormat(
                    key_value_pair.to_string(),
                ));
            }
        }
        // If we succeeded, above (i.e. there were no invalid or unsupported keywords),
        // return the connection string builder.
        Ok(connection_string_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApplicationIntent;
    use rstest;

    #[rstest::rstest]
    #[case("Application Intent=ReadWrite", "Application Intent=ReadWrite")]
    #[case("ApplicationIntent=ReadOnly", "Application Intent=ReadOnly")]
    #[case("Application Name=Some Name", "Application Name=Some Name")]
    #[case("App=Some Name", "Application Name=Some Name")]
    #[case("AttachDbFileName=SomeFile.txt", "AttachDbFilename=SomeFile.txt")]
    #[case("Initial File Name=SomeFile.txt", "AttachDbFilename=SomeFile.txt")]
    #[case(
        "Authentication=ActiveDirectoryPassword",
        "Authentication=Active Directory Password"
    )]
    #[case(
        "Column Encryption Setting=Enabled",
        "Column Encryption Setting=Enabled"
    )]
    #[case("Command Timeout=160", "Command Timeout=160")]
    #[case("Connect Retry Count=50", "Connect Retry Count=50")]
    #[case("ConnectRetryCount=50", "Connect Retry Count=50")]
    #[case("Connect Retry Interval=50", "Connect Retry Interval=50")]
    #[case("ConnectRetryInterval=50", "Connect Retry Interval=50")]
    #[case("Connect Timeout=100", "Connect Timeout=100")]
    #[case("Connection Timeout=100", "Connect Timeout=100")]
    #[case("Timeout=100", "Connect Timeout=100")]
    #[case("Current Language=en", "Current Language=en")]
    #[case("Language=en", "Current Language=en")]
    #[case("Data Source=127.0.0.1:1433", "Data Source=127.0.0.1:1433")]
    #[case("Addr=127.0.0.1:1433", "Data Source=127.0.0.1:1433")]
    #[case("Address=127.0.0.1:1433", "Data Source=127.0.0.1:1433")]
    #[case("Network Address=127.0.0.1:1433", "Data Source=127.0.0.1:1433")]
    #[case("Server=127.0.0.1:1433", "Data Source=127.0.0.1:1433")]
    #[case(
        "Enclave Attestation Url=https://someurl",
        "Enclave Attestation Url=https://someurl"
    )]
    #[case("Encrypt=Yes", "Encrypt=True")]
    #[case("Enlist=No", "Enlist=False")]
    #[case("Failover Partner=Some Value", "Failover Partner=Some Value")]
    #[case("Initial Catalog=SomeDb", "Initial Catalog=SomeDb")]
    #[case("Database=SomeDb", "Initial Catalog=SomeDb")]
    #[case("Integrated Security=SSPI", "Integrated Security=True")]
    #[case("Integrated Security=True", "Integrated Security=True")]
    #[case("Trusted_Connection=True", "Integrated Security=True")]
    #[case("IP Address Preference=IPv4First", "IP Address Preference=IPv4First")]
    #[case("IPAddressPreference=IPv6First", "IP Address Preference=IPv6First")]
    #[case("Load Balance Timeout=123", "Load Balance Timeout=123")]
    #[case("Connection Lifetime=123", "Load Balance Timeout=123")]
    #[case("Max Pool Size=10", "Max Pool Size=10")]
    #[case("Min Pool Size=10", "Min Pool Size=10")]
    #[case("Multiple Active Result Sets=True", "Multiple Active Result Sets=True")]
    #[case("MultipleActiveResultSets=True", "Multiple Active Result Sets=True")]
    #[case("Multi Subnet Failover=No", "Multi Subnet Failover=False")]
    #[case("MultiSubnetFailover=No", "Multi Subnet Failover=False")]
    #[case("Packet Size=10", "Packet Size=10")]
    #[case("Password=abc123", "Password=abc123")]
    #[case("Pwd=abc123", "Password=abc123")]
    #[case("Persist Security Info=No", "Persist Security Info=False")]
    #[case("PersistSecurityInfo=No", "Persist Security Info=False")]
    #[case("Pooling=Yes", "Pooling=True")]
    #[case("Pool Blocking Period=AlwaysBlock", "Pool Blocking Period=AlwaysBlock")]
    #[case("PoolBlockingPeriod=AlwaysBlock", "Pool Blocking Period=AlwaysBlock")]
    #[case("Replication=yes", "Replication=True")]
    #[case("Transaction Binding=ABC", "Transaction Binding=ABC")]
    #[case("Trust Server Certificate=Yes", "Trust Server Certificate=True")]
    #[case("TrustServerCertificate=No", "Trust Server Certificate=False")]
    #[case("Type System Version=123", "Type System Version=123")]
    #[case("User ID=domain\\user", "User ID=domain\\user")]
    #[case("User=domain\\user", "User ID=domain\\user")]
    #[case("UID=domain\\user", "User ID=domain\\user")]
    #[case("Workstation ID=ABC", "Workstation ID=ABC")]
    #[case("WSID=ABC", "Workstation ID=ABC")]
    #[case("User Instance=True", "User Instance=True")]
    #[case(
        "Data Source=127.0.0.1:1433;Initial Catalog=SomeDb;User ID=domain\\user;Password=abc123",
        "Data Source=127.0.0.1:1433;Initial Catalog=SomeDb;User ID=domain\\user;Password=abc123"
    )]
    fn test_connection_string_roundtrip(#[case] value: &str, #[case] expected: &str) {
        // Create a connection string builder
        let builder: SqlConnectionStringBuilder = value.try_into().unwrap();
        // Have it build a string
        let actual = builder.connection_string();
        // Check the results
        assert_eq!(expected, actual.as_str());
    }

    fn do_stuff() {
        let builder: SqlConnectionStringBuilder = "".try_into().unwrap();
        let a = builder.application_intent();
        let b = builder.application_intent();
    }
}
