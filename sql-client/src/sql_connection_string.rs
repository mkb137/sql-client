use crate::db_connection_string_defaults::DbConnectionStringDefaults;
use crate::db_connection_string_keywords::DbConnectionStringKeywordsLower;
use crate::db_connection_string_utils::{
    convert_to_boolean, convert_to_integrated_security, get_local_db_instance_name_from_server_name,
};
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
    /// ?
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
