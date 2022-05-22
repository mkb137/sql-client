//! Creates a SQL connection string.
//!
//!
use crate::sql_client_error::SqlClientError;
use crate::{
    ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod,
    SqlConnectionColumnEncryptionSetting, SqlConnectionIpAddressPreference,
};
use std::collections::HashMap;

// Defaults
const DEFAULT_APPLICATION_INTENT: ApplicationIntent = ApplicationIntent::ReadWrite;
const DEFAULT_APPLICATION_NAME: &str = "SqlClient Data Provider";
const DEFAULT_ATTACH_DB_FILENAME: Option<String> = None;
const DEFAULT_AUTHENTICATION: SqlAuthenticationMethod = SqlAuthenticationMethod::NotSpecified;
const DEFAULT_COLUMN_ENCRYPTION_SETTING: SqlConnectionColumnEncryptionSetting =
    SqlConnectionColumnEncryptionSetting::Disabled;
const DEFAULT_CONNECT_RETRY_COUNT: u8 = 1;
const DEFAULT_CONNECT_RETRY_INTERVAL: u8 = 10;
const DEFAULT_CONNECT_TIMEOUT: u16 = 15;
const DEFAULT_COMMAND_TIMEOUT: u16 = 30;
const DEFAULT_CURRENT_LANGUAGE: Option<String> = None;
const DEFAULT_DATA_SOURCE: Option<String> = None;
const DEFAULT_ENCLAVE_ATTESTATION_URL: Option<String> = None;
const DEFAULT_ENCRYPT: bool = true;
const DEFAULT_ENLIST: bool = true;
const DEFAULT_FAILOVER_PARTNER: Option<String> = None;
const DEFAULT_INITIAL_CATALOG: Option<String> = None;
const DEFAULT_INTEGRATED_SECURITY: bool = false;
const DEFAULT_IP_ADDRESS_PREFERENCE: SqlConnectionIpAddressPreference =
    SqlConnectionIpAddressPreference::IPv4First;
const DEFAULT_LOAD_BALANCE_TIMEOUT: u16 = 0;
const DEFAULT_MAX_POOL_SIZE: u8 = 100;
const DEFAULT_MIN_POOL_SIZE: u8 = 0;
const DEFAULT_MULTIPLE_ACTIVE_RESULT_SETS: bool = false;
const DEFAULT_MULTI_SUBNET_FAILOVER: bool = false;
const DEFAULT_PACKET_SIZE: u16 = 8000;
const DEFAULT_PASSWORD: Option<String> = None;
const DEFAULT_PERSIST_SECURITY_INFO: bool = false;
const DEFAULT_POOLING: bool = true;
const DEFAULT_POOL_BLOCKING_PERIOD: PoolBlockingPeriod = PoolBlockingPeriod::Auto;
const DEFAULT_REPLICATION: bool = false;
const DEFAULT_TRANSACTION_BINDING: &str = "Implicit Unbind";
const DEFAULT_TYPE_SYSTEM_VERSION: &str = "Latest";
const DEFAULT_USER_ID: Option<String> = None;
const DEFAULT_WORKSTATION_ID: Option<String> = None;
const DEFAULT_TRUST_SERVER_CERTIFICATE: bool = false;
const DEFAULT_USER_INSTANCE: bool = false;

/// Converts a string value to an application intent.
fn convert_to_application_intent(value: &str) -> Result<ApplicationIntent, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "readonly" => Ok(ApplicationIntent::ReadOnly),
        "readwrite" => Ok(ApplicationIntent::ReadWrite),
        _ => Err(SqlClientError::UnsupportedValue(
            "SqlClientError".to_string(),
            value.to_string(),
        )),
    }
}

/// Converts a true/false/yes/no string to a boolean.
fn convert_to_boolean(value: &str) -> Result<bool, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(SqlClientError::UnsupportedValue(
            "boolean".to_string(),
            value.to_string(),
        )),
    }
}

fn convert_to_column_encryption_setting(
    value: &str,
) -> Result<SqlConnectionColumnEncryptionSetting, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "disabled" => Ok(SqlConnectionColumnEncryptionSetting::Disabled),
        "enabled" => Ok(SqlConnectionColumnEncryptionSetting::Enabled),
        _ => Err(SqlClientError::UnsupportedValue(
            "SqlConnectionColumnEncryptionSetting".to_string(),
            value.to_string(),
        )),
    }
}

fn convert_to_integrated_security(value: &str) -> Result<bool, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "true" | "yes" | "sspi" => Ok(true),
        "false" | "no" => Ok(false),
        _ => Err(SqlClientError::UnsupportedValue(
            "Integrated Security".to_string(),
            value.to_string(),
        )),
    }
}

fn convert_to_ip_address_preference(
    value: &str,
) -> Result<SqlConnectionIpAddressPreference, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "ipv4first" => Ok(SqlConnectionIpAddressPreference::IPv4First),
        "ipv6first" => Ok(SqlConnectionIpAddressPreference::IPv6First),
        "useplatformdefault" => Ok(SqlConnectionIpAddressPreference::UsePlatformDefault),
        _ => Err(SqlClientError::UnsupportedValue(
            "IP Address Preference".to_string(),
            value.to_string(),
        )),
    }
}

fn convert_to_pool_blocking_period(value: &str) -> Result<PoolBlockingPeriod, SqlClientError> {
    match value.trim().to_lowercase().as_str() {
        "auto" => Ok(PoolBlockingPeriod::Auto),
        "alwaysblock" => Ok(PoolBlockingPeriod::AlwaysBlock),
        "neverblock" => Ok(PoolBlockingPeriod::NeverBlock),
        _ => Err(SqlClientError::UnsupportedValue(
            "Pool blocking period".to_string(),
            value.to_string(),
        )),
    }
}
fn convert_to_authentication_method(
    value: &str,
) -> Result<SqlAuthenticationMethod, SqlClientError> {
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

        _ => Err(SqlClientError::UnsupportedValue(
            "Authentication Method".to_string(),
            value.to_string(),
        )),
    }
}

///
struct SqlConnectionStringBuilder {
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
    password: Option<String>,
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
    /// Indicates the type system the application expects.
    type_system_version: String,
    /// The user ID to be used when connecting to SQL Server.
    user_id: Option<String>,
    /// The name of the workstation connecting to SQL Server.
    workstation_id: Option<String>,
    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    trust_server_certificate: bool,
    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    user_instance: bool,
}

impl SqlConnectionStringBuilder {
    /// Declares the application workload type when connecting to a database in an SQL Server Availability Group.
    fn set_application_intent(&mut self, value: ApplicationIntent) {
        self.application_intent = value;
    }
    /// The name of the application associated with the connection string.
    fn set_application_name(&mut self, value: String) {
        self.application_name = value;
    }
    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    fn set_attach_db_filename(&mut self, value: Option<String>) {
        self.attach_db_filename = value;
    }
    /// ?
    fn set_authentication(&mut self, value: SqlAuthenticationMethod) {
        self.authentication = value;
    }
    /// ?
    fn set_column_encryption_setting(&mut self, value: SqlConnectionColumnEncryptionSetting) {
        self.column_encryption_setting = value;
    }
    /// The length of time (in seconds) to wait for a command to the server before terminating the attempt and generating an error.
    fn set_command_timeout(&mut self, value: u16) {
        self.command_timeout = value;
    }
    /// The number of reconnections attempted after identifying that there was an idle connection failure. This must be an integer between 0 and 255. Default is 1. Set to 0 to disable reconnecting on idle connection failures.
    fn set_connect_retry_count(&mut self, value: u8) {
        self.connect_retry_count = value;
    }
    /// Amount of time (in seconds) between each reconnection attempt after identifying that there was an idle connection failure. This must be an integer between 1 and 60. The default is 10 seconds.
    fn set_connect_retry_interval(&mut self, value: u8) {
        self.connect_retry_interval = value;
    }
    /// The length of time (in seconds) to wait for a connection to the server before terminating the attempt and generating an error.
    fn set_connect_timeout(&mut self, value: u16) {
        self.connect_timeout = value;
    }
    /// The SQL Server Language record name.
    fn set_current_language(&mut self, value: Option<String>) {
        self.current_language = value;
    }
    /// The name or network address of the instance of SQL Server to connect to.
    fn set_data_source(&mut self, value: Option<String>) {
        self.data_source = value;
    }
    /// ?
    fn set_enclave_attestation_url(&mut self, value: Option<String>) {
        self.enclave_attestation_url = value;
    }
    /// Whether SQL Server uses SSL encryption for all data sent between the client and server if the server has a certificate installed.
    fn set_encrypt(&mut self, value: bool) {
        self.encrypt = value;
    }
    /// Whether the SQL Server connection pooler automatically enlists the connection in the creation thread's current transaction context.
    fn set_enlist(&mut self, value: bool) {
        self.enlist = value;
    }
    /// The name or address of the partner server to connect to if the primary server is down.
    fn set_failover_partner(&mut self, value: Option<String>) {
        self.failover_partner = value;
    }
    /// The name of the database associated with the connection.
    fn set_initial_catalog(&mut self, value: Option<String>) {
        self.initial_catalog = value;
    }
    /// Whether User ID and Password are specified in the connection (when false) or whether the current Windows account credentials are used for authentication (when true).
    fn set_integrated_security(&mut self, value: bool) {
        self.integrated_security = value;
    }
    /// ?
    fn set_ip_address_preference(&mut self, value: SqlConnectionIpAddressPreference) {
        self.ip_address_preference = value;
    }
    /// ??
    fn set_load_balance_timeout(&mut self, value: u16) {
        self.load_balance_timeout = value;
    }
    /// The maximum number of connections allowed in the connection pool for this specific connection string.
    fn set_max_pool_size(&mut self, value: u8) {
        self.max_pool_size = value;
    }
    /// The minimum number of connections allowed in the connection pool for this specific connection string.
    fn set_min_pool_size(&mut self, value: u8) {
        self.min_pool_size = value;
    }
    /// When true, an application can maintain multiple active result sets (MARS). When false, an application must process or cancel all result sets from one batch before it can execute any other batch on that connection.
    fn set_multiple_active_result_sets(&mut self, value: bool) {
        self.multiple_active_result_sets = value;
    }
    /// If your application is connecting to an Always On availability group (AG) or Always On Failover Cluster Instance (FCI) on different subnets, setting MultiSubnetFailover=true provides faster detection of and connection to the (currently) active server.
    fn set_multi_subnet_failover(&mut self, value: bool) {
        self.multi_subnet_failover = value;
    }
    /// The size in bytes of the network packets used to communicate with an instance of SQL Server.
    fn set_packet_size(&mut self, value: u16) {
        self.packet_size = value;
    }
    /// The password for the SQL Server account.
    fn set_password(&mut self, value: Option<String>) {
        self.password = value;
    }
    /// Indicates if security-sensitive information, such as the password or access token, should be returned as part of the connection string on a connection created with this SqlConnectionStringBuilder after that connection has ever been in an open state.
    fn set_persist_security_info(&mut self, value: bool) {
        self.persist_security_info = value;
    }
    /// Whether the connection will be pooled or explicitly opened every time that the connection is requested.
    fn set_pooling(&mut self, value: bool) {
        self.pooling = value;
    }
    /// The blocking period behavior for a connection pool.
    fn set_pool_blocking_period(&mut self, value: PoolBlockingPeriod) {
        self.pool_blocking_period = value;
    }
    /// Whether replication is supported using the connection.
    fn set_replication(&mut self, value: bool) {
        self.replication = value;
    }
    /// Indicates how the connection maintains its association with an enlisted System.Transactions transaction.
    fn set_transaction_binding(&mut self, value: String) {
        self.transaction_binding = value;
    }
    /// Indicates the type system the application expects.
    fn set_type_system_version(&mut self, value: String) {
        self.type_system_version = value;
    }
    /// The user ID to be used when connecting to SQL Server.
    fn set_user_id(&mut self, value: Option<String>) {
        self.user_id = value;
    }
    /// The name of the workstation connecting to SQL Server.
    fn set_workstation_id(&mut self, value: Option<String>) {
        self.workstation_id = value;
    }
    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    fn set_trust_server_certificate(&mut self, value: bool) {
        self.trust_server_certificate = value;
    }
    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    fn set_user_instance(&mut self, value: bool) {
        self.user_instance = value;
    }
}

impl Default for SqlConnectionStringBuilder {
    fn default() -> Self {
        Self {
            application_intent: DEFAULT_APPLICATION_INTENT,
            application_name: DEFAULT_APPLICATION_NAME.to_string(),
            attach_db_filename: DEFAULT_ATTACH_DB_FILENAME,
            authentication: DEFAULT_AUTHENTICATION,
            column_encryption_setting: DEFAULT_COLUMN_ENCRYPTION_SETTING,
            command_timeout: DEFAULT_COMMAND_TIMEOUT,
            connect_retry_count: DEFAULT_CONNECT_RETRY_COUNT,
            connect_retry_interval: DEFAULT_CONNECT_RETRY_INTERVAL,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            current_language: DEFAULT_CURRENT_LANGUAGE,
            data_source: DEFAULT_DATA_SOURCE,
            enclave_attestation_url: DEFAULT_ENCLAVE_ATTESTATION_URL,
            encrypt: DEFAULT_ENCRYPT,
            enlist: DEFAULT_ENLIST,
            failover_partner: DEFAULT_FAILOVER_PARTNER,
            initial_catalog: DEFAULT_INITIAL_CATALOG,
            integrated_security: DEFAULT_INTEGRATED_SECURITY,
            ip_address_preference: DEFAULT_IP_ADDRESS_PREFERENCE,
            load_balance_timeout: DEFAULT_LOAD_BALANCE_TIMEOUT,
            max_pool_size: DEFAULT_MAX_POOL_SIZE,
            min_pool_size: DEFAULT_MIN_POOL_SIZE,
            multiple_active_result_sets: DEFAULT_MULTIPLE_ACTIVE_RESULT_SETS,
            multi_subnet_failover: DEFAULT_MULTI_SUBNET_FAILOVER,
            packet_size: DEFAULT_PACKET_SIZE,
            password: DEFAULT_PASSWORD,
            persist_security_info: DEFAULT_PERSIST_SECURITY_INFO,
            pooling: DEFAULT_POOLING,
            pool_blocking_period: DEFAULT_POOL_BLOCKING_PERIOD,
            replication: DEFAULT_REPLICATION,
            transaction_binding: DEFAULT_TRANSACTION_BINDING.to_string(),
            type_system_version: DEFAULT_TYPE_SYSTEM_VERSION.to_string(),
            user_id: DEFAULT_USER_ID,
            workstation_id: DEFAULT_WORKSTATION_ID,
            trust_server_certificate: DEFAULT_TRUST_SERVER_CERTIFICATE,
            user_instance: DEFAULT_USER_INSTANCE,
        }
    }
}

impl TryFrom<&str> for SqlConnectionStringBuilder {
    type Error = SqlClientError;

    /// Parses a connection string into a connection string builder.
    fn try_from(connection_string: &str) -> Result<Self, Self::Error> {
        log::debug!("try_from - connection_string = {:?}", connection_string);
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
                let value = key.trim();
                log::debug!(" - got keyword '{:?}', value = '{:?}'", keyword, value);
                // Check the keyword against the keywords we know
                match keyword {
                    "application intent" => {
                        let application_intent = convert_to_application_intent(value)?;
                        connection_string_builder.set_application_intent(application_intent);
                    }
                    "application name" => {
                        connection_string_builder.set_application_name(value.to_string());
                    }
                    "attachdbfilename" => {
                        connection_string_builder.set_attach_db_filename(Some(value.to_string()));
                    }
                    "authentication" => {
                        connection_string_builder.set_attach_db_filename(Some(value.to_string()));
                    }
                    "column encryption setting" => {
                        let column_encrpytion_setting =
                            convert_to_column_encryption_setting(value)?;
                        connection_string_builder
                            .set_column_encryption_setting(column_encrpytion_setting);
                    }
                    "command timeout" => {
                        let command_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_command_timeout(command_timeout);
                    }
                    "connect retry count" => {
                        let connect_retry_count: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_connect_retry_count(connect_retry_count);
                    }
                    "connect retry interval" => {
                        let connect_retry_interval: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                        connection_string_builder
                            .set_connect_retry_interval(connect_retry_interval);
                    }
                    "connect timeout" => {
                        let connect_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_connect_timeout(connect_timeout);
                    }
                    "current language" => {
                        connection_string_builder.set_current_language(Some(value.to_string()));
                    }
                    "data source" => {
                        connection_string_builder.set_data_source(Some(value.to_string()));
                    }
                    "enclave attestation url" => {
                        connection_string_builder
                            .set_enclave_attestation_url(Some(value.to_string()));
                    }
                    "encrypt" => {
                        let encrypt = convert_to_boolean(value)?;
                        connection_string_builder.set_encrypt(encrypt);
                    }
                    "enlist" => {
                        let enlist = convert_to_boolean(value)?;
                        connection_string_builder.set_enlist(enlist);
                    }
                    "failover partner" => {
                        connection_string_builder.set_failover_partner(Some(value.to_string()));
                    }
                    "initial catalog" => {
                        connection_string_builder.set_initial_catalog(Some(value.to_string()));
                    }
                    "integrated security" => {
                        let integrated_security = convert_to_integrated_security(value)?;
                        connection_string_builder.set_integrated_security(integrated_security);
                    }
                    "IP Address Preference" => {
                        let ip_address_preference = convert_to_ip_address_preference(value)?;
                        connection_string_builder.set_ip_address_preference(ip_address_preference);
                    }
                    "load balance timeout" => {
                        let load_balance_timeout: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_load_balance_timeout(load_balance_timeout);
                    }
                    "max pool size" => {
                        let max_pool_size: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_max_pool_size(max_pool_size);
                    }
                    "min pool size" => {
                        let min_pool_size: u8 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_min_pool_size(min_pool_size);
                    }
                    "multiple active result sets" => {
                        let multiple_active_result_sets = convert_to_boolean(value)?;
                        connection_string_builder
                            .set_multiple_active_result_sets(multiple_active_result_sets);
                    }
                    "multi subnet failover" => {
                        let multi_subnet_failover = convert_to_boolean(value)?;
                        connection_string_builder.set_multi_subnet_failover(multi_subnet_failover);
                    }
                    "packet size" => {
                        let packet_size: u16 = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                        connection_string_builder.set_packet_size(packet_size);
                    }
                    "password" => {
                        connection_string_builder.set_password(Some(value.to_string()));
                    }
                    "persist security info" => {
                        let persist_security_info = convert_to_boolean(value)?;
                        connection_string_builder.set_persist_security_info(persist_security_info);
                    }
                    "pooling" => {
                        let pooling = convert_to_boolean(value)?;
                        connection_string_builder.set_pooling(pooling);
                    }
                    "pool blocking period" => {
                        let pool_blocking_period = convert_to_pool_blocking_period(value)?;
                        connection_string_builder.set_pool_blocking_period(pool_blocking_period);
                    }
                    "replication" => {
                        let replication = convert_to_boolean(value)?;
                        connection_string_builder.set_replication(replication);
                    }
                    "transaction binding" => {
                        connection_string_builder.set_transaction_binding(value.to_string());
                    }
                    "type system version" => {
                        connection_string_builder.set_type_system_version(value.to_string());
                    }
                    "user id" => {
                        connection_string_builder.set_user_id(Some(value.to_string()));
                    }
                    "workstation id" => {
                        connection_string_builder.set_workstation_id(Some(value.to_string()));
                    }
                    "trust server certificate" => {
                        let trust_server_certificate = convert_to_boolean(value)?;
                        connection_string_builder
                            .set_trust_server_certificate(trust_server_certificate);
                    }
                    "user instance" => {
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
        // If we succeeded, above, return the connection string builder.
        Ok(connection_string_builder)
    }
    // /// Parses a connection string to create a connection string builder.
    // fn from(connection_string: &str) -> Self {
    //     log::debug!("from - connection_string = {:?}", connection_string);
    //     // Create a dictionary of keys and values
    //     connection_string.split(";").iter()
    //     let mut dict = HashMap::new();
    //     //
    //     todo!()
    // }
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
    #[case("auto", PoolBlockingPeriod::Auto)]
    #[case("AUTO", PoolBlockingPeriod::Auto)]
    #[case(" AuTo ", PoolBlockingPeriod::Auto)]
    #[case("alwaysblock", PoolBlockingPeriod::AlwaysBlock)]
    #[case("AlwaysBlock", PoolBlockingPeriod::AlwaysBlock)]
    #[case(" AlwaysBlock ", PoolBlockingPeriod::AlwaysBlock)]
    #[case("neverblock", PoolBlockingPeriod::NeverBlock)]
    #[case("NeverBlock", PoolBlockingPeriod::NeverBlock)]
    #[case(" NeverBlock ", PoolBlockingPeriod::NeverBlock)]
    fn test_convert_to_pool_blocking_period(
        #[case] value: &str,
        #[case] expected: PoolBlockingPeriod,
    ) {
        match convert_to_pool_blocking_period(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

    #[rstest::rstest]
    #[case("readwrite", ApplicationIntent::ReadWrite)]
    #[case("ReadWrite", ApplicationIntent::ReadWrite)]
    #[case("readonly", ApplicationIntent::ReadOnly)]
    #[case("ReadOnly", ApplicationIntent::ReadOnly)]
    fn test_convert_to_application_intent(
        #[case] value: &str,
        #[case] expected: ApplicationIntent,
    ) {
        match convert_to_application_intent(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }

    #[rstest::rstest]
    #[case("SqlPassword", SqlAuthenticationMethod::SqlPassword)]
    #[case("Sql Password", SqlAuthenticationMethod::SqlPassword)]
    #[case(
        "ActiveDirectoryManagedIdentity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "Active Directory Managed Identity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "ActiveDirectoryIntegrated",
        SqlAuthenticationMethod::ActiveDirectoryIntegrated
    )]
    #[case(
        "Active Directory Integrated",
        SqlAuthenticationMethod::ActiveDirectoryIntegrated
    )]
    #[case(
        "ActiveDirectoryInteractive",
        SqlAuthenticationMethod::ActiveDirectoryInteractive
    )]
    #[case(
        "Active Directory Interactive",
        SqlAuthenticationMethod::ActiveDirectoryInteractive
    )]
    #[case(
        "ActiveDirectoryServicePrincipal",
        SqlAuthenticationMethod::ActiveDirectoryServicePrincipal
    )]
    #[case(
        "Active Directory Service Principal",
        SqlAuthenticationMethod::ActiveDirectoryServicePrincipal
    )]
    #[case(
        "ActiveDirectoryDeviceCodeFlow",
        SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow
    )]
    #[case(
        "Active Directory Device Code Flow",
        SqlAuthenticationMethod::ActiveDirectoryDeviceCodeFlow
    )]
    #[case(
        "ActiveDirectoryManagedIdentity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case(
        "Active Directory Managed Identity",
        SqlAuthenticationMethod::ActiveDirectoryManagedIdentity
    )]
    #[case("ActiveDirectoryMSI", SqlAuthenticationMethod::ActiveDirectoryMSI)]
    #[case("Active Directory MSI", SqlAuthenticationMethod::ActiveDirectoryMSI)]
    #[case(
        "ActiveDirectoryDefault",
        SqlAuthenticationMethod::ActiveDirectoryDefault
    )]
    #[case(
        "Active Directory Default",
        SqlAuthenticationMethod::ActiveDirectoryDefault
    )]
    #[case("SqlCertificate", SqlAuthenticationMethod::SqlCertificate)]
    #[case("Sql Certificate", SqlAuthenticationMethod::SqlCertificate)]
    fn test_convert_to_authentication_method(
        #[case] value: &str,
        #[case] expected: SqlAuthenticationMethod,
    ) {
        match convert_to_authentication_method(value) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(e) => assert!(false, "Expected: Ok, Actual: Err"),
        }
    }
}
