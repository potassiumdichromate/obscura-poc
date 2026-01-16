// src/escrow.rs - STRUCTS ONLY (NO IMPLEMENTATIONS)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowAccount {
    pub escrow_account_id: String,
    pub buyer_account_id: String,
    pub seller_account_id: String,
    pub amount: u64,
    pub status: EscrowStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EscrowStatus {
    Created,
    Funded,
    Released,
    Refunded,
}

// THAT'S IT - NO impl BLOCKS AT ALL