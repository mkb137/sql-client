use crate::db_connection_string_defaults::DbConnectionStringDefaults;
use crate::db_connection_string_keywords::DbConnectionStringKeywordsLower;
use crate::db_connection_string_utils::{
    convert_to_boolean, convert_to_integrated_security, get_local_db_instance_name_from_server_name,
};
use crate::sql_credential::SqlCredential;
use crate::{
    ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod, SqlClientError,
    SqlConnectionAttestationProtocol, SqlConnectionColumnEncryptionSetting,
    SqlConnectionIpAddressPreference, TransactionBinding, TypeSystem,
};
use secstr::SecStr;

/// A parsed connection string.  Very similar to [ConnectionStringBuilder].
pub(crate) struct SqlConnectionString {
    /// Declares the application workload type when connecting to a database in an SQL Server Availability Group.
    application_intent: ApplicationIntent,
    /// The name of the application associated with the connection string.
    application_name: String,
    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    attach_db_filename: Option<String>,
    /// ???
    attestation_protocol: SqlConnectionAttestationProtocol,
    /// The authentication type
    auth_type: SqlAuthenticationMethod,
    /// ???
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
    /// ???
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
    /// ???
    ip_address_preference: SqlConnectionIpAddressPreference,
    /// ???
    load_balance_timeout: u16,
    /// Created based on datasource, set to None if datasource is not LocalDB
    local_db_instance: Option<String>,
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
    transaction_binding: TransactionBinding,
    /// Whether the channel will be encrypted while bypassing walking the certificate chain to validate trust.
    trust_server_certificate: bool,
    /// Indicates the type system the application expects.
    type_system_version: String,
    /// Indicates the type system the application expects.
    type_system_assembly_version: TypeSystem,
    /// The user ID to be used when connecting to SQL Server.
    user_id: Option<String>,
    /// Gets or sets a value that indicates whether to redirect the connection from the default SQL Server Express instance to a runtime-initiated instance running under the account of the caller.
    user_instance: bool,
    /// The name of the workstation connecting to SQL Server.
    workstation_id: Option<String>,
    /// Expanded during construction so that CreatePermissionSet & Expand are consistent
    expanded_attach_db_filename: Option<String>,
}

impl SqlConnectionString {
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

    /// ???
    pub fn auth_type(&self) -> SqlAuthenticationMethod {
        self.auth_type.clone()
    }

    /// ???
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

    /// ???
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

    /// Gets or sets a string that contains the name of the primary data file. This includes the full path name of an attachable database.
    pub fn expanded_attach_db_filename(&self) -> Option<String> {
        self.expanded_attach_db_filename.clone()
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

    /// ???
    pub fn ip_address_preference(&self) -> SqlConnectionIpAddressPreference {
        self.ip_address_preference.clone()
    }
    /// ???
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
    pub fn transaction_binding(&self) -> TransactionBinding {
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

    /// The SQL credentials, if there's a user ID and password
    pub fn sql_credential(&self) -> Result<Option<SqlCredential>, SqlClientError> {
        // Check the user ID and password that may have been provided in the connection string.
        match (self.user_id(), self.password()) {
            // If we have both a user ID and password...
            (Some(user_id), Some(password)) => {
                // Try to create credentials.
                let sql_credential = SqlCredential::new(user_id, password)?;
                // If successful, return them.
                Ok(Some(sql_credential))
            }
            // If we don't have both username and password, return an error.
            _ => Ok(None),
        }
    }
}

impl TryFrom<&str> for SqlConnectionString {
    type Error = SqlClientError;

    /// Parses a connection string into a connection string builder.
    fn try_from(connection_string: &str) -> Result<Self, Self::Error> {
        // The connection string struct is immutable, so we'll first set defaults
        // for all properties and then override the values supplied in the given string.
        let mut application_intent: ApplicationIntent =
            DbConnectionStringDefaults::APPLICATION_INTENT;
        let mut application_name: String = DbConnectionStringDefaults::APPLICATION_NAME.to_string();
        let mut attach_db_filename: Option<String> = DbConnectionStringDefaults::ATTACH_DB_FILENAME;
        let mut attestation_protocol: SqlConnectionAttestationProtocol =
            DbConnectionStringDefaults::ATTESTATION_PROTOCOL;
        let mut auth_type: SqlAuthenticationMethod = DbConnectionStringDefaults::AUTHENTICATION;
        let mut column_encryption_setting: SqlConnectionColumnEncryptionSetting =
            DbConnectionStringDefaults::COLUMN_ENCRYPTION_SETTING;
        let mut connect_retry_count: u8 = DbConnectionStringDefaults::CONNECT_RETRY_COUNT;
        let mut connect_retry_interval: u8 = DbConnectionStringDefaults::CONNECT_RETRY_INTERVAL;
        let mut connect_timeout: u16 = DbConnectionStringDefaults::CONNECT_TIMEOUT;
        let mut command_timeout: u16 = DbConnectionStringDefaults::COMMAND_TIMEOUT;
        let mut current_language: Option<String> = DbConnectionStringDefaults::CURRENT_LANGUAGE;
        let mut data_source: Option<String> = DbConnectionStringDefaults::DATA_SOURCE;
        let mut enclave_attestation_url: Option<String> =
            DbConnectionStringDefaults::ENCLAVE_ATTESTATION_URL;
        let mut encrypt: bool = DbConnectionStringDefaults::ENCRYPT;
        let mut enlist: bool = DbConnectionStringDefaults::ENLIST;
        let mut failover_partner: Option<String> = DbConnectionStringDefaults::FAILOVER_PARTNER;
        let mut initial_catalog: Option<String> = DbConnectionStringDefaults::INITIAL_CATALOG;
        let mut integrated_security: bool = DbConnectionStringDefaults::INTEGRATED_SECURITY;
        let mut ip_address_preference: SqlConnectionIpAddressPreference =
            DbConnectionStringDefaults::IP_ADDRESS_PREFERENCE;
        let mut load_balance_timeout: u16 = DbConnectionStringDefaults::LOAD_BALANCE_TIMEOUT;
        let mut local_db_instance: Option<String> = DbConnectionStringDefaults::LOCAL_DB_INSTANCE;
        let mut max_pool_size: u8 = DbConnectionStringDefaults::MAX_POOL_SIZE;
        let mut min_pool_size: u8 = DbConnectionStringDefaults::MIN_POOL_SIZE;
        let mut multiple_active_result_sets: bool =
            DbConnectionStringDefaults::MULTIPLE_ACTIVE_RESULT_SETS;
        let mut multi_subnet_failover: bool = DbConnectionStringDefaults::MULTI_SUBNET_FAILOVER;
        let mut packet_size: u16 = DbConnectionStringDefaults::PACKET_SIZE;
        let mut password: Option<SecStr> = DbConnectionStringDefaults::PASSWORD;
        let mut persist_security_info: bool = DbConnectionStringDefaults::PERSIST_SECURITY_INFO;
        let mut pooling: bool = DbConnectionStringDefaults::POOLING;
        let mut pool_blocking_period: PoolBlockingPeriod =
            DbConnectionStringDefaults::POOL_BLOCKING_PERIOD;
        let mut replication: bool = DbConnectionStringDefaults::REPLICATION;
        let mut transaction_binding: TransactionBinding =
            DbConnectionStringDefaults::TRANSACTION_BINDING;
        let mut trust_server_certificate: bool =
            DbConnectionStringDefaults::TRUST_SERVER_CERTIFICATE;
        let mut type_system_version: String =
            DbConnectionStringDefaults::TYPE_SYSTEM_VERSION.to_string();
        let mut type_system_assembly_version: TypeSystem =
            DbConnectionStringDefaults::TYPE_SYSTEM_ASSEMBLY_VERSION;
        let mut user_id: Option<String> = DbConnectionStringDefaults::USER_ID;
        let mut user_instance: bool = DbConnectionStringDefaults::USER_INSTANCE;
        let mut workstation_id: Option<String> = DbConnectionStringDefaults::WORKSTATION_ID;
        let mut expanded_attach_db_filename: Option<String> = None;

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
                        application_intent = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::APPLICATION_NAME
                    | DbConnectionStringKeywordsLower::APPLICATION_NAME_ALT => {
                        application_name = value.to_string();
                    }
                    DbConnectionStringKeywordsLower::ATTACH_DB_FILENAME
                    | DbConnectionStringKeywordsLower::ATTACH_DB_FILENAME_ALT => {
                        attach_db_filename = Some(value.to_string());
                        // In .NET, this includes an application domain.
                        // As far as I know, we don't have anything like that.  For now, copy it.
                        expanded_attach_db_filename = attach_db_filename.clone();
                    }
                    DbConnectionStringKeywordsLower::ATTESTATION_PROTOCOL => {
                        attestation_protocol = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::AUTHENTICATION => {
                        auth_type = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::COLUMN_ENCRYPTION_SETTING => {
                        column_encryption_setting = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::COMMAND_TIMEOUT => {
                        command_timeout = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::CONNECT_RETRY_COUNT
                    | DbConnectionStringKeywordsLower::CONNECT_RETRY_COUNT_ALT => {
                        connect_retry_count = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::CONNECT_RETRY_INTERVAL
                    | DbConnectionStringKeywordsLower::CONNECT_RETRY_INTERVAL_ALT => {
                        connect_retry_interval = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u8".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::CONNECT_TIMEOUT
                    | DbConnectionStringKeywordsLower::CONNECT_TIMEOUT_ALT1
                    | DbConnectionStringKeywordsLower::CONNECT_TIMEOUT_ALT2 => {
                        connect_timeout = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::CURRENT_LANGUAGE
                    | DbConnectionStringKeywordsLower::CURRENT_LANGUAGE_ALT => {
                        current_language = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::DATA_SOURCE
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT1
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT2
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT3
                    | DbConnectionStringKeywordsLower::DATA_SOURCE_ALT4 => {
                        data_source = Some(value.to_string());
                        // Get the instance name from the data source, if it has one
                        local_db_instance = get_local_db_instance_name_from_server_name(value);
                    }
                    DbConnectionStringKeywordsLower::ENCLAVE_ATTESTATION_URL => {
                        enclave_attestation_url = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::ENCRYPT => {
                        encrypt = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::ENLIST => {
                        enlist = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::FAILOVER_PARTNER => {
                        failover_partner = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::INITIAL_CATALOG
                    | DbConnectionStringKeywordsLower::INITIAL_CATALOG_ALT => {
                        initial_catalog = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::INTEGRATED_SECURITY
                    | DbConnectionStringKeywordsLower::INTEGRATED_SECURITY_ALT => {
                        integrated_security = convert_to_integrated_security(value)?;
                    }
                    DbConnectionStringKeywordsLower::IP_ADDRESS_PREFERENCE
                    | DbConnectionStringKeywordsLower::IP_ADDRESS_PREFERENCE_ALT => {
                        ip_address_preference = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::LOAD_BALANCE_TIMEOUT
                    | DbConnectionStringKeywordsLower::LOAD_BALANCE_TIMEOUT_ALT => {
                        load_balance_timeout = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::MAX_POOL_SIZE => {
                        max_pool_size = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::MIN_POOL_SIZE => {
                        min_pool_size = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::MULTIPLE_ACTIVE_RESULT_SETS
                    | DbConnectionStringKeywordsLower::MULTIPLE_ACTIVE_RESULT_SETS_ALT => {
                        multiple_active_result_sets = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::MULTI_SUBNET_FAILOVER
                    | DbConnectionStringKeywordsLower::MULTI_SUBNET_FAILOVER_ALT => {
                        multi_subnet_failover = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::PACKET_SIZE => {
                        packet_size = value.parse().map_err(|_| {
                            SqlClientError::UnsupportedValue("u16".to_string(), value.to_string())
                        })?;
                    }
                    DbConnectionStringKeywordsLower::PASSWORD
                    | DbConnectionStringKeywordsLower::PASSWORD_ALT => {
                        password = Some(SecStr::from(value));
                    }
                    DbConnectionStringKeywordsLower::PERSIST_SECURITY_INFO
                    | DbConnectionStringKeywordsLower::PERSIST_SECURITY_INFO_ALT => {
                        persist_security_info = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::POOLING => {
                        pooling = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::POOL_BLOCKING_PERIOD
                    | DbConnectionStringKeywordsLower::POOL_BLOCKING_PERIOD_ALT => {
                        pool_blocking_period = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::REPLICATION => {
                        replication = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::TRANSACTION_BINDING => {
                        transaction_binding = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::TRUST_SERVER_CERTIFICATE
                    | DbConnectionStringKeywordsLower::TRUST_SERVER_CERTIFICATE_ALT => {
                        trust_server_certificate = convert_to_boolean(value)?;
                    }
                    DbConnectionStringKeywordsLower::TYPE_SYSTEM_VERSION => {
                        type_system_version = value.to_string();
                        type_system_assembly_version = value.try_into()?;
                    }
                    DbConnectionStringKeywordsLower::USER_ID
                    | DbConnectionStringKeywordsLower::USER_ID_ALT1
                    | DbConnectionStringKeywordsLower::USER_ID_ALT2 => {
                        user_id = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::WORKSTATION_ID
                    | DbConnectionStringKeywordsLower::WORKSTATION_ALT_ID => {
                        workstation_id = Some(value.to_string());
                    }
                    DbConnectionStringKeywordsLower::USER_INSTANCE => {
                        user_instance = convert_to_boolean(value)?;
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
        // If we were able to parse the values without error, create a new connection string struct
        let sql_connection_string = SqlConnectionString {
            application_intent,
            application_name,
            attach_db_filename,
            attestation_protocol,
            auth_type,
            column_encryption_setting,
            connect_retry_count,
            connect_retry_interval,
            connect_timeout,
            command_timeout,
            current_language,
            data_source,
            enclave_attestation_url,
            encrypt,
            enlist,
            failover_partner,
            initial_catalog,
            integrated_security,
            ip_address_preference,
            load_balance_timeout,
            local_db_instance,
            max_pool_size,
            min_pool_size,
            multiple_active_result_sets,
            multi_subnet_failover,
            packet_size,
            password,
            persist_security_info,
            pooling,
            pool_blocking_period,
            replication,
            transaction_binding,
            trust_server_certificate,
            type_system_version,
            type_system_assembly_version,
            user_id,
            user_instance,
            workstation_id,
            expanded_attach_db_filename,
        };
        // Return it
        Ok(sql_connection_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_application_intent() {
        let connection_string: SqlConnectionString =
            "Application Intent=ReadWrite".try_into().unwrap();
        assert_eq!(
            ApplicationIntent::ReadWrite,
            connection_string.application_intent
        )
    }

    #[test]
    pub fn test_parse_application_name() {
        let connection_string: SqlConnectionString =
            "Application Name=Some App".try_into().unwrap();
        assert_eq!("Some App".to_string(), connection_string.application_name)
    }

    #[test]
    pub fn test_parse_attach_db_filename() {
        let connection_string: SqlConnectionString =
            "AttachDbFilename=SomeFile".try_into().unwrap();
        assert_eq!(
            Some("SomeFile".to_string()),
            connection_string.attach_db_filename
        )
    }

    #[test]
    pub fn test_parse_attestation_protocol() {
        let connection_string: SqlConnectionString = "Attestation Protocol=AAS".try_into().unwrap();
        assert_eq!(
            SqlConnectionAttestationProtocol::AAS,
            connection_string.attestation_protocol
        )
    }

    #[test]
    pub fn test_parse_auth_type() {
        let connection_string: SqlConnectionString =
            "Authentication=SqlPassword".try_into().unwrap();
        assert_eq!(
            SqlAuthenticationMethod::SqlPassword,
            connection_string.auth_type
        )
    }

    #[test]
    pub fn test_parse_column_encryption_setting() {
        let connection_string: SqlConnectionString =
            "Column Encryption Setting=Enabled".try_into().unwrap();
        assert_eq!(
            SqlConnectionColumnEncryptionSetting::Enabled,
            connection_string.column_encryption_setting
        )
    }

    #[test]
    pub fn test_parse_connect_retry_count() {
        let connection_string: SqlConnectionString = "Connect Retry Count=111".try_into().unwrap();
        assert_eq!(111u8, connection_string.connect_retry_count)
    }

    #[test]
    pub fn test_parse_connect_retry_interval() {
        let connection_string: SqlConnectionString =
            "Connect Retry Interval=22".try_into().unwrap();
        assert_eq!(22u8, connection_string.connect_retry_interval)
    }

    #[test]
    pub fn test_parse_connect_timeout() {
        let connection_string: SqlConnectionString = "Connect Timeout=33".try_into().unwrap();
        assert_eq!(33u16, connection_string.connect_timeout)
    }

    #[test]
    pub fn test_parse_command_timeout() {
        let connection_string: SqlConnectionString = "Command Timeout=44".try_into().unwrap();
        assert_eq!(44u16, connection_string.command_timeout)
    }

    #[test]
    pub fn test_parse_current_language() {
        let connection_string: SqlConnectionString = "Current Language=English".try_into().unwrap();
        assert_eq!(
            Some("English".to_string()),
            connection_string.current_language
        )
    }

    #[test]
    pub fn test_parse_data_source() {
        let connection_string: SqlConnectionString =
            "Data Source=(localdb)\\SOME_NAME".try_into().unwrap();
        assert_eq!(
            Some("(localdb)\\SOME_NAME".to_string()),
            connection_string.data_source
        );
        assert_eq!(
            Some("SOME_NAME".to_string()),
            connection_string.local_db_instance
        );
    }

    #[test]
    pub fn test_parse_enclave_attestation_url() {
        let connection_string: SqlConnectionString = "Attestation Protocol=HGS".try_into().unwrap();
        assert_eq!(
            SqlConnectionAttestationProtocol::HGS,
            connection_string.attestation_protocol
        )
    }

    #[test]
    pub fn test_parse_encrypt() {
        let connection_string: SqlConnectionString = "Encrypt=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.encrypt)
    }

    #[test]
    pub fn test_parse_enlist() {
        let connection_string: SqlConnectionString = "Enlist=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.enlist)
    }

    #[test]
    pub fn test_parse_failover_partner() {
        let connection_string: SqlConnectionString = "Failover Partner=fp".try_into().unwrap();
        assert_eq!(Some("fp".to_string()), connection_string.failover_partner)
    }

    #[test]
    pub fn test_parse_initial_catalog() {
        let connection_string: SqlConnectionString = "Initial Catalog=SomeDb".try_into().unwrap();
        assert_eq!(
            Some("SomeDb".to_string()),
            connection_string.initial_catalog
        )
    }

    #[test]
    pub fn test_parse_integrated_security() {
        let connection_string: SqlConnectionString = "Integrated Security=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.integrated_security)
    }

    #[test]
    pub fn test_parse_ip_address_preference() {
        let connection_string: SqlConnectionString =
            "IP Address Preference=IPv6First".try_into().unwrap();
        assert_eq!(
            SqlConnectionIpAddressPreference::IPv6First,
            connection_string.ip_address_preference
        )
    }

    #[test]
    pub fn test_parse_load_balance_timeout() {
        let connection_string: SqlConnectionString = "Load Balance Timeout=55".try_into().unwrap();
        assert_eq!(55u16, connection_string.load_balance_timeout)
    }

    #[test]
    pub fn test_parse_max_pool_size() {
        let connection_string: SqlConnectionString = "Max Pool Size=11".try_into().unwrap();
        assert_eq!(11u8, connection_string.max_pool_size)
    }

    #[test]
    pub fn test_parse_min_pool_size() {
        let connection_string: SqlConnectionString = "Min Pool Size=2".try_into().unwrap();
        assert_eq!(2u8, connection_string.min_pool_size)
    }

    #[test]
    pub fn test_parse_multiple_active_result_sets() {
        let connection_string: SqlConnectionString =
            "Multiple Active Result Sets=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.multiple_active_result_sets)
    }

    #[test]
    pub fn test_parse_multi_subnet_failover() {
        let connection_string: SqlConnectionString =
            "Multi Subnet Failover=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.multi_subnet_failover)
    }

    #[test]
    pub fn test_parse_packet_size() {
        let connection_string: SqlConnectionString = "Packet Size=123".try_into().unwrap();
        assert_eq!(123u16, connection_string.packet_size)
    }

    #[test]
    pub fn test_parse_password() {
        let connection_string: SqlConnectionString = "Password=abc123".try_into().unwrap();
        assert_eq!(Some(SecStr::from("abc123")), connection_string.password)
    }

    #[test]
    pub fn test_parse_persist_security_info() {
        let connection_string: SqlConnectionString =
            "Persist Security Info=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.persist_security_info)
    }

    #[test]
    pub fn test_parse_pooling() {
        let connection_string: SqlConnectionString =
            "Application Intent=ReadWrite".try_into().unwrap();
        assert_eq!(
            ApplicationIntent::ReadWrite,
            connection_string.application_intent
        )
    }

    #[test]
    pub fn test_parse_pool_blocking_period() {
        let connection_string: SqlConnectionString = "Pooling=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.pooling)
    }

    #[test]
    pub fn test_parse_replication() {
        let connection_string: SqlConnectionString = "Replication=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.replication)
    }

    #[test]
    pub fn test_parse_transaction_binding() {
        let connection_string: SqlConnectionString =
            "Transaction Binding=Explicit Unbind".try_into().unwrap();
        assert_eq!(
            TransactionBinding::ExplicitUnbind,
            connection_string.transaction_binding
        )
    }

    #[test]
    pub fn test_parse_trust_server_certificate() {
        let connection_string: SqlConnectionString =
            "Trust Server Certificate=Yes".try_into().unwrap();
        assert_eq!(true, connection_string.trust_server_certificate)
    }

    #[test]
    pub fn test_parse_type_system_version() {
        let connection_string: SqlConnectionString =
            "Type System Version=SQL Server 2005".try_into().unwrap();
        assert_eq!(
            TypeSystem::SqlServer2005,
            connection_string.type_system_assembly_version
        );
        assert_eq!(
            "SQL Server 2005".to_string(),
            connection_string.type_system_version
        );
    }

    #[test]
    pub fn test_parse_user_id() {
        let connection_string: SqlConnectionString = "User ID=Some User".try_into().unwrap();
        assert_eq!(Some("Some User".to_string()), connection_string.user_id)
    }

    #[test]
    pub fn test_parse_user_instance() {
        let connection_string: SqlConnectionString = "User Instance=True".try_into().unwrap();
        assert_eq!(true, connection_string.user_instance)
    }

    #[test]
    pub fn test_parse_workstation_id() {
        let connection_string: SqlConnectionString =
            "Application Intent=ReadWrite".try_into().unwrap();
        assert_eq!(
            ApplicationIntent::ReadWrite,
            connection_string.application_intent
        )
    }

    #[test]
    pub fn test_parse_expanded_attach_db_filename() {
        let connection_string: SqlConnectionString = "Workstation ID=ABC".try_into().unwrap();
        assert_eq!(Some("ABC".to_string()), connection_string.workstation_id)
    }
}
