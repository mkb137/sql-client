#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod application_intent;
pub mod pool_blocking_period;
pub mod sql_authentication_method;
pub mod sql_client_error;
pub mod sql_column_encryption_setting;
pub mod sql_connection;
pub mod sql_connection_ip_address_preference;
pub mod sql_connection_string_builder;
pub mod sql_credential;
pub(crate) mod tds_enums;
mod test_init;

// Re-exports
pub use application_intent::ApplicationIntent;
pub use pool_blocking_period::PoolBlockingPeriod;
pub use sql_authentication_method::SqlAuthenticationMethod;
pub use sql_client_error::SqlClientError;
pub use sql_column_encryption_setting::SqlConnectionColumnEncryptionSetting;
pub use sql_connection_ip_address_preference::SqlConnectionIpAddressPreference;
pub use sql_connection_string_builder::SqlConnectionStringBuilder;
