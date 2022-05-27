use crate::{
    ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod,
    SqlConnectionColumnEncryptionSetting, SqlConnectionIpAddressPreference,
};
use secstr::SecStr;
pub(crate) struct DbConnectionStringDefaults;
impl DbConnectionStringDefaults {
    pub const DEFAULT_APPLICATION_INTENT: ApplicationIntent = ApplicationIntent::ReadWrite;
    pub const DEFAULT_APPLICATION_NAME: &'static str = "SqlClient Data Provider";
    pub const DEFAULT_ATTACH_DB_FILENAME: Option<String> = None;
    pub const DEFAULT_AUTHENTICATION: SqlAuthenticationMethod =
        SqlAuthenticationMethod::NotSpecified;
    pub const DEFAULT_COLUMN_ENCRYPTION_SETTING: SqlConnectionColumnEncryptionSetting =
        SqlConnectionColumnEncryptionSetting::Disabled;
    pub const DEFAULT_CONNECT_RETRY_COUNT: u8 = 1;
    pub const DEFAULT_CONNECT_RETRY_INTERVAL: u8 = 10;
    pub const DEFAULT_CONNECT_TIMEOUT: u16 = 15;
    pub const DEFAULT_COMMAND_TIMEOUT: u16 = 30;
    pub const DEFAULT_CURRENT_LANGUAGE: Option<String> = None;
    pub const DEFAULT_DATA_SOURCE: Option<String> = None;
    pub const DEFAULT_ENCLAVE_ATTESTATION_URL: Option<String> = None;
    pub const DEFAULT_ENCRYPT: bool = true;
    pub const DEFAULT_ENLIST: bool = true;
    pub const DEFAULT_FAILOVER_PARTNER: Option<String> = None;
    pub const DEFAULT_INITIAL_CATALOG: Option<String> = None;
    pub const DEFAULT_INTEGRATED_SECURITY: bool = false;
    pub const DEFAULT_IP_ADDRESS_PREFERENCE: SqlConnectionIpAddressPreference =
        SqlConnectionIpAddressPreference::IPv4First;
    pub const DEFAULT_LOAD_BALANCE_TIMEOUT: u16 = 0;
    pub const DEFAULT_MAX_POOL_SIZE: u8 = 100;
    pub const DEFAULT_MIN_POOL_SIZE: u8 = 0;
    pub const DEFAULT_MULTIPLE_ACTIVE_RESULT_SETS: bool = false;
    pub const DEFAULT_MULTI_SUBNET_FAILOVER: bool = false;
    pub const DEFAULT_PACKET_SIZE: u16 = 8000;
    pub const DEFAULT_PASSWORD: Option<SecStr> = None;
    pub const DEFAULT_PERSIST_SECURITY_INFO: bool = false;
    pub const DEFAULT_POOLING: bool = true;
    pub const DEFAULT_POOL_BLOCKING_PERIOD: PoolBlockingPeriod = PoolBlockingPeriod::Auto;
    pub const DEFAULT_REPLICATION: bool = false;
    pub const DEFAULT_TRANSACTION_BINDING: &'static str = "Implicit Unbind";
    pub const DEFAULT_TRUST_SERVER_CERTIFICATE: bool = false;
    pub const DEFAULT_TYPE_SYSTEM_VERSION: &'static str = "Latest";
    pub const DEFAULT_USER_ID: Option<String> = None;
    pub const DEFAULT_USER_INSTANCE: bool = false;
    pub const DEFAULT_WORKSTATION_ID: Option<String> = None;
}
