#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod application_intent;
mod pool_blocking_period;
mod sql_authentication_method;
mod sql_client_error;
mod sql_column_encryption_setting;
mod sql_connection_ip_address_preference;
mod sql_connection_string_builder;
mod test_init;

pub(crate) use application_intent::ApplicationIntent;
pub(crate) use pool_blocking_period::PoolBlockingPeriod;
pub(crate) use sql_authentication_method::SqlAuthenticationMethod;
pub(crate) use sql_column_encryption_setting::SqlConnectionColumnEncryptionSetting;
pub(crate) use sql_connection_ip_address_preference::SqlConnectionIpAddressPreference;

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
