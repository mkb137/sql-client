#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod application_intent;
pub(crate) mod connection_state;
pub(crate) mod db_connection_internal;
pub(crate) mod db_connection_pool;
pub(crate) mod db_connection_string_defaults;
pub(crate) mod db_connection_string_keywords;
pub(crate) mod db_connection_string_utils;
pub mod pool_blocking_period;
mod retry_enumerators;
pub mod sql_authentication_method;
pub mod sql_client_error;
pub mod sql_column_encryption_setting;
pub mod sql_connection;
mod sql_connection_attestation_protocol;
pub mod sql_connection_ip_address_preference;
mod sql_connection_string;
pub mod sql_connection_string_builder;
pub mod sql_credential;
pub(crate) mod tds_enums;
mod test_init;
mod transaction;
mod transaction_binding;
mod type_system;

// Re-exports
#[doc(inline)]
pub use application_intent::ApplicationIntent;
#[doc(inline)]
pub use pool_blocking_period::PoolBlockingPeriod;
#[doc(inline)]
pub use sql_authentication_method::SqlAuthenticationMethod;
#[doc(inline)]
pub use sql_client_error::SqlClientError;
#[doc(inline)]
pub use sql_column_encryption_setting::SqlConnectionColumnEncryptionSetting;
#[doc(inline)]
pub use sql_connection::SqlConnection;
#[doc(inline)]
pub(crate) use sql_connection_attestation_protocol::SqlConnectionAttestationProtocol;
#[doc(inline)]
pub use sql_connection_ip_address_preference::SqlConnectionIpAddressPreference;
#[doc(inline)]
pub use sql_connection_string_builder::SqlConnectionStringBuilder;
#[doc(inline)]
pub use sql_credential::SqlCredential;
#[doc(inline)]
pub(crate) use transaction_binding::{TransactionBinding, TransactionBindingKeywords};
#[doc(inline)]
pub(crate) use type_system::{TypeSystem, TypeSystemVersion};
