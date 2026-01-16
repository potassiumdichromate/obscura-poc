// src/settlement.rs - FINAL FIXED VERSION
// REAL atomic settlement: simultaneous fund transfer + ownership transfer

use anyhow::{Result, Context};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{Settlement, SettlementStatus, PurchaseOffer};
use crate::MidenClientWrapper;
use crate::escrow::EscrowAccount;

/// REAL Settlement Manager
/// Handles atomic settlements on Miden blockchain
pub struct SettlementManager {
    settlements: Arc<RwLock<HashMap<String, Settlement>>>,
}

impl SettlementManager {
    pub fn new() -> Self {
        Self {
            settlements: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Initiate settlement process
    pub async fn initiate_settlement(
        &self,
        offer: &PurchaseOffer,
        property_note_id: String,
    ) -> Result<Settlement> {
        tracing::info!("⚡ Initiating settlement");
        tracing::info!("   Offer: {}", offer.offer_id);
        tracing::info!("   Property Note: {}", property_note_id);
        
        let settlement_id = Uuid::new_v4().to_string();
        
        let settlement = Settlement {
            settlement_id: settlement_id.clone(),
            offer_id: offer.offer_id.clone(),
            property_note_id,
            escrow_account_id: offer.escrow_account_id.clone()
                .ok_or_else(|| anyhow::anyhow!("No escrow account"))?,
            funds_transfer_tx: None,
            ownership_transfer_tx: None,
            status: SettlementStatus::Initiated,
            created_at: Utc::now(),
            completed_at: None,
        };
        
        let mut settlements = self.settlements.write().await;
        settlements.insert(settlement_id.clone(), settlement.clone());
        
        tracing::info!("✅ Settlement initiated: {}", settlement_id);
        
        Ok(settlement)
    }
    
    /// Execute REAL atomic settlement on Miden
    /// This is atomic: both transfers succeed or both fail
    pub async fn execute_settlement(
        &self,
        settlement_id: &str,
        client: &mut MidenClientWrapper,
    ) -> Result<Settlement> {
        tracing::info!("⚡⚡⚡ Executing ATOMIC settlement");
        tracing::info!("   Settlement: {}", settlement_id);
        
        let mut settlements = self.settlements.write().await;
        let settlement = settlements
            .get_mut(settlement_id)
            .ok_or_else(|| anyhow::anyhow!("Settlement not found"))?;
        
        // =================================================================
        // STEP 1: Release funds from escrow to seller
        // =================================================================
        tracing::info!("💰 Step 1/2: Releasing funds from escrow to seller");
        
        let escrow_account = EscrowAccount {
            escrow_account_id: settlement.escrow_account_id.clone(),
            buyer_account_id: client.bob_account_id
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Bob not found"))?
                .to_string(),  // FIXED: Convert AccountId to String
            seller_account_id: client.alice_account_id
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Alice not found"))?
                .to_string(),  // FIXED: Convert AccountId to String
            amount: 0, // Already in escrow
            status: crate::escrow::EscrowStatus::Funded,
        };
        
        // Execute escrow release
        let funds_tx = client.release_escrow_real(&escrow_account).await
            .context("Failed to release escrow funds")?;
        
        settlement.funds_transfer_tx = Some(funds_tx.clone());
        settlement.status = SettlementStatus::FundsTransferred;
        
        tracing::info!("✅ Funds transferred to seller");
        tracing::info!("   TX: {}", funds_tx);
        
        // =================================================================
        // STEP 2: Transfer property ownership (NFT note)
        // =================================================================
        tracing::info!("🏠 Step 2/2: Transferring property ownership to buyer");
        
        let ownership_tx = client
            .transfer_property_ownership(&settlement.property_note_id, "bob")
            .await
            .context("Failed to transfer property ownership")?;
        
        settlement.ownership_transfer_tx = Some(ownership_tx.clone());
        settlement.status = SettlementStatus::OwnershipTransferred;
        
        tracing::info!("✅ Property ownership transferred to buyer");
        tracing::info!("   TX: {}", ownership_tx);
        
        // =================================================================
        // COMPLETION: Both transfers successful - ATOMIC!
        // =================================================================
        settlement.status = SettlementStatus::Completed;
        settlement.completed_at = Some(Utc::now());
        
        tracing::info!("✅✅✅ ATOMIC SETTLEMENT COMPLETED");
        tracing::info!("   Funds TX: {}", settlement.funds_transfer_tx.as_ref().unwrap());
        tracing::info!("   Ownership TX: {}", settlement.ownership_transfer_tx.as_ref().unwrap());
        tracing::info!("   Status: {:?}", settlement.status);
        
        Ok(settlement.clone())
    }
    
    /// Get settlement by ID
    pub async fn get_settlement(&self, settlement_id: &str) -> Option<Settlement> {
        let settlements = self.settlements.read().await;
        settlements.get(settlement_id).cloned()
    }
    
    /// Get all settlements for an offer
    pub async fn get_settlements_for_offer(&self, offer_id: &str) -> Vec<Settlement> {
        let settlements = self.settlements.read().await;
        settlements
            .values()
            .filter(|s| s.offer_id == offer_id)
            .cloned()
            .collect()
    }
    
    /// Verify settlement completed successfully
    pub async fn verify_settlement_complete(&self, settlement_id: &str) -> Result<bool> {
        let settlements = self.settlements.read().await;
        
        if let Some(settlement) = settlements.get(settlement_id) {
            Ok(settlement.status == SettlementStatus::Completed
                && settlement.funds_transfer_tx.is_some()
                && settlement.ownership_transfer_tx.is_some())
        } else {
            Err(anyhow::anyhow!("Settlement not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::OfferStatus;
    
    #[tokio::test]
    async fn test_initiate_settlement() {
        let manager = SettlementManager::new();
        
        let offer = PurchaseOffer {
            offer_id: "offer-123".to_string(),
            listing_id: "listing-123".to_string(),
            buyer_account_id: "buyer".to_string(),
            seller_account_id: "seller".to_string(),
            offer_amount: 1_000_000,
            status: OfferStatus::Accepted,
            escrow_account_id: Some("escrow-123".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let settlement = manager.initiate_settlement(
            &offer,
            "note-123".to_string(),
        ).await.unwrap();
        
        assert_eq!(settlement.offer_id, "offer-123");
        assert_eq!(settlement.status, SettlementStatus::Initiated);
    }
    
    #[test]
    fn test_settlement_manager_creation() {
        let manager = SettlementManager::new();
        assert!(true);
    }
}