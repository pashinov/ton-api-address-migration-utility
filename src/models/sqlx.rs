use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TransactionDb {
    pub id: Uuid,
    pub service_id: ServiceId,
    pub message_hash: String,
    pub transaction_hash: Option<String>,
    pub transaction_lt: Option<BigDecimal>,
    pub transaction_timeout: Option<i64>,
    pub transaction_scan_lt: Option<i64>,
    pub transaction_timestamp: Option<NaiveDateTime>,
    pub sender_workchain_id: Option<i32>,
    pub sender_hex: Option<String>,
    pub account_workchain_id: i32,
    pub account_hex: String,
    pub messages: Option<serde_json::Value>,
    pub messages_hash: Option<serde_json::Value>,
    pub data: Option<serde_json::Value>,
    pub original_value: Option<BigDecimal>,
    pub original_outputs: Option<serde_json::Value>,
    pub value: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub balance_change: Option<BigDecimal>,
    pub direction: TonTransactionDirection,
    pub status: TonTransactionStatus,
    pub error: Option<String>,
    pub aborted: bool,
    pub bounce: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct AddressDb {
    pub id: Uuid,
    pub service_id: ServiceId,
    pub workchain_id: i32,
    pub hex: String,
    pub base64url: String,
    pub public_key: String,
    pub private_key: String,
    pub account_type: AccountType,
    pub custodians: Option<i32>,
    pub confirmations: Option<i32>,
    pub custodians_public_keys: Option<serde_json::Value>,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
