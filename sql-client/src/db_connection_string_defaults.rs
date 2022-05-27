use crate::{
    ApplicationIntent, PoolBlockingPeriod, SqlAuthenticationMethod,
    SqlConnectionColumnEncryptionSetting, SqlConnectionIpAddressPreference,
};
use secstr::SecStr;
pub(crate) struct DbConnectionStringDefaults;
impl DbConnectionStringDefaults {
    pub const APPLICATION_INTENT: ApplicationIntent = ApplicationIntent::ReadWrite;
    pub const APPLICATION_NAME: &'static str = "SqlClient Data Provider";
    pub const ATTACH_DB_FILENAME: Option<String> = None;
    pub const AUTHENTICATION: SqlAuthenticationMethod = SqlAuthenticationMethod::NotSpecified;
    pub const COLUMN_ENCRYPTION_SETTING: SqlConnectionColumnEncryptionSetting =
        SqlConnectionColumnEncryptionSetting::Disabled;
    pub const CONNECT_RETRY_COUNT: u8 = 1;
    pub const CONNECT_RETRY_INTERVAL: u8 = 10;
    pub const CONNECT_TIMEOUT: u16 = 15;
    pub const COMMAND_TIMEOUT: u16 = 30;
    pub const CURRENT_LANGUAGE: Option<String> = None;
    pub const DATA_SOURCE: Option<String> = None;
    pub const ENCLAVE_ATTESTATION_URL: Option<String> = None;
    pub const ENCRYPT: bool = true;
    pub const ENLIST: bool = true;
    pub const FAILOVER_PARTNER: Option<String> = None;
    pub const INITIAL_CATALOG: Option<String> = None;
    pub const INTEGRATED_SECURITY: bool = false;
    pub const IP_ADDRESS_PREFERENCE: SqlConnectionIpAddressPreference =
        SqlConnectionIpAddressPreference::IPv4First;
    pub const LOAD_BALANCE_TIMEOUT: u16 = 0;
    pub const MAX_POOL_SIZE: u8 = 100;
    pub const MIN_POOL_SIZE: u8 = 0;
    pub const MULTIPLE_ACTIVE_RESULT_SETS: bool = false;
    pub const MULTI_SUBNET_FAILOVER: bool = false;
    pub const PACKET_SIZE: u16 = 8000;
    pub const PASSWORD: Option<SecStr> = None;
    pub const PERSIST_SECURITY_INFO: bool = false;
    pub const POOLING: bool = true;
    pub const POOL_BLOCKING_PERIOD: PoolBlockingPeriod = PoolBlockingPeriod::Auto;
    pub const REPLICATION: bool = false;
    pub const TRANSACTION_BINDING: &'static str = "Implicit Unbind";
    pub const TRUST_SERVER_CERTIFICATE: bool = false;
    pub const TYPE_SYSTEM_VERSION: &'static str = "Latest";
    pub const USER_ID: Option<String> = None;
    pub const USER_INSTANCE: bool = false;
    pub const WORKSTATION_ID: Option<String> = None;
}
