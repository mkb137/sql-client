#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod sql_client_error;
mod sql_connection_string_builder;

/// ?
#[derive(PartialEq, Debug)]
enum ApplicationIntent {
    /// The application workload type when connecting to a server is read write.
    ReadWrite = 0,
    /// The application workload type when connecting to a server is read only.
    ReadOnly = 1,
}

/// Describes the different SQL authentication methods that can be used by a client connecting to Azure SQL Database
#[derive(PartialEq, Debug)]
enum SqlAuthenticationMethod {
    /// The authentication method is not specified.
    NotSpecified,
    /// The authentication method is Sql Password.
    SqlPassword,
    /// The authentication method uses Active Directory Password. Use Active Directory Password to connect to a SQL Database using an Azure AD principal name and password.
    ActiveDirectoryPassword,
    /// The authentication method uses Active Directory Integrated. Use Active Directory Integrated to connect to a SQL Database using integrated Windows authentication.
    ActiveDirectoryIntegrated,
    /// The authentication method uses Active Directory Interactive. Available since .NET Framework 4.7.2.
    ActiveDirectoryInteractive,
    ActiveDirectoryServicePrincipal,
    ActiveDirectoryDeviceCodeFlow,
    ActiveDirectoryManagedIdentity,
    ActiveDirectoryMSI,
    ActiveDirectoryDefault,
    SqlCertificate,
}

/// ?
#[derive(PartialEq, Debug)]
enum SqlConnectionColumnEncryptionSetting {
    Disabled = 0,
    Enabled = 1,
}

/// ?
#[derive(PartialEq, Debug)]
enum SqlConnectionAttestationProtocol {
    NotSpecified = 0,
    AAS = 1,
    None = 2,
    HGS = 3,
}

/// ?
#[derive(PartialEq, Debug)]
enum SqlConnectionIpAddressPreference {
    IPv4First = 0, // default
    IPv6First = 1,
    UsePlatformDefault = 2,
}

/// ?
#[derive(PartialEq, Debug)]
enum PoolBlockingPeriod {
    // Blocking period OFF for Azure SQL servers, but ON for all other SQL servers.
    Auto = 0,
    // Blocking period ON for all SQL servers including Azure SQL servers.
    AlwaysBlock = 1,
    // Blocking period OFF for all SQL servers including Azure SQL servers.
    NeverBlock = 2,
}

/// ?
mod db_connection_string_defaults {}

/// ?
mod db_connection_string_keywords {
    // SqlClient
    const APPLICATION_INTENT: &str = "Application Intent";
    const APPLICATION_NAME: &str = "Application Name";
    const ATTACH_DB_FILENAME: &str = "AttachDbFilename";
    const ATTESTATION_PROTOCOL: &str = "Attestation Protocol";
    const AUTHENTICATION: &str = "Authentication";
    const COLUMN_ENCRYPTION_SETTING: &str = "Column Encryption Setting";
    const COMMAND_TIMEOUT: &str = "Command Timeout";
    const CONNECT_RETRY_COUNT: &str = "Connect Retry Count";
    const CONNECT_RETRY_INTERVAL: &str = "Connect Retry Interval";
    const CONNECT_TIMEOUT: &str = "Connect Timeout";
    const CONNECTION_RESET: &str = "Connection Reset";
    const CONTEXT_CONNECTION: &str = "Context Connection";
    const CURRENT_LANGUAGE: &str = "Current Language";
    const ENCLAVE_ATTESTATION_URL: &str = "Enclave Attestation Url";
    const ENCRYPT: &str = "Encrypt";
    const FAILOVER_PARTNER: &str = "Failover Partner";
    const INITIAL_CATALOG: &str = "Initial Catalog";
    const IP_ADDRESS_PREFERENCE: &str = "IP Address Preference";
    const MULTIPLE_ACTIVE_RESULT_SETS: &str = "Multiple Active Result Sets";
    const MULTI_SUBNET_FAILOVER: &str = "Multi Subnet Failover";
    const NETWORK_LIBRARY: &str = "Network Library";
    const PACKET_SIZE: &str = "Packet Size";
    const REPLICATION: &str = "Replication";
    const TRANSACTION_BINDING: &str = "Transaction Binding";
    const TRUST_SERVER_CERTIFICATE: &str = "Trust Server Certificate";
    const TYPE_SYSTEM_VERSION: &str = "Type System Version";
    const USER_INSTANCE: &str = "User Instance";
    const WORKSTATION_ID: &str = "Workstation ID";

    // common keywords (OleDb, OracleClient, SqlClient)
    const DATA_SOURCE: &str = "Data Source";
    const INTEGRATED_SECURITY: &str = "Integrated Security";
    const PASSWORD: &str = "Password";
    const PERSIST_SECURITY_INFO: &str = "Persist Security Info";
    const USER_ID: &str = "User ID";

    // managed pooling (OracleClient, SqlClient)
    const ENLIST: &str = "Enlist";
    const LOAD_BALANCE_TIMEOUT: &str = "Load Balance Timeout";
    const MAX_POOL_SIZE: &str = "Max Pool Size";
    const MIN_POOL_SIZE: &str = "Min Pool Size";
    const POOLING: &str = "Pooling";
    const POOL_BLOCKING_PERIOD: &str = "Pool Blocking Period";
}

/// ?
mod db_connection_string_synonyms {
    const ADDR: &str = "addr";
    const ADDRESS: &str = "address";
    const APP: &str = "app";
    const APPLICATION_INTENT: &str = "applicationintent";
    const CONNECTION_TIMEOUT: &str = "connection timeout";
    const CONNECT_RETRY_COUNT: &str = "connectretrycount";
    const CONNECT_RETRY_INTERVAL: &str = "connectretryinterval";
    const CONNECTION_LIFETIME: &str = "connection lifetime";
    const DATABASE: &str = "database";
    const EXTENDED_PROPERTIES: &str = "extended properties";
    const INITIAL_FILENAME: &str = "initial file name";
    const IP_ADDRESS_PREFERENCE: &str = "ipaddresspreference";
    const LANGUAGE: &str = "language";
    const MULTIPLE_ACTIVE_RESULT_SETS: &str = "multipleactiveresultsets";
    const MULTI_SUBNET_FAILOVER: &str = "multisubnetfailover";
    const NET: &str = "net";
    const NETWORK: &str = "network";
    const NETWORK_ADDRESS: &str = "network address";
    const PERSIST_SECURITY_INFO: &str = "persistsecurityinfo";
    const POOL_BLOCKING_PERIOD: &str = "poolblockingperiod";
    const PWD: &str = "pwd";
    const TIMEOUT: &str = "timeout";
    const SERVER: &str = "server";
    const TRUSTED_CONNECTION: &str = "trusted_connection"; // underscore introduced in everett
    const TRUST_SERVER_CERTIFICATE: &str = "trustservercertificate";
    const UID: &str = "uid";
    const USER: &str = "user";
    const WSID: &str = "wsid";
}
