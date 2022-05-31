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
        return Err(SqlClientError::ArgumentNull("".to_string()));
    }
}
