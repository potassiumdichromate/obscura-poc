// src/models.rs
// Data models for API

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyListing {
    pub listing_id: String,
    pub property_id: String,
    pub owner_account_id: String,
    pub note_id: String,
    pub ipfs_cid: String,
    pub status: ListingStatus,
    pub selective_disclosure: SelectiveDisclosure,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListingStatus {
    Active,
    UnderOffer,
    Sold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectiveDisclosure {
    pub show_valuation_to_accredited: bool,
    pub show_documents_to_verified: bool,
    pub show_location_to_eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOffer {
    pub offer_id: String,
    pub listing_id: String,
    pub buyer_account_id: String,
    pub seller_account_id: String,
    pub offer_amount: u64,
    pub status: OfferStatus,
    pub escrow_account_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OfferStatus {
    Pending,
    Accepted,
    Rejected,
    EscrowFunded,
    Settled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofEvent {
    pub event_id: String,
    pub account_id: String,
    pub proof_type: String,
    pub status: ProofStatus,
    pub program_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProofStatus {
    Generated,
    Verified,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub account_id: String,
    pub account_type: String,
    pub is_connected: bool,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDetails {
    pub property_id: String,
    pub title: String,
    pub description: String,
    pub property_type: String,
    pub valuation: Option<u64>,
    pub price: u64,
    pub location: Option<String>,
    pub square_feet: u32,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub year_built: u16,
    pub owner_name: String,
    pub legal_description: Option<String>,
    pub tax_id: Option<String>,
    pub zoning: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub settlement_id: String,
    pub offer_id: String,
    pub property_note_id: String,
    pub escrow_account_id: String,
    pub funds_transfer_tx: Option<String>,
    pub ownership_transfer_tx: Option<String>,
    pub status: SettlementStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettlementStatus {
    Initiated,
    FundsTransferred,
    OwnershipTransferred,
    Completed,
    Failed,
}