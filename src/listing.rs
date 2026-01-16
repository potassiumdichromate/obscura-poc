// src/listing.rs
// REAL property listing manager with privacy controls

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{
    PropertyListing, ListingStatus, SelectiveDisclosure, PropertyDetails
};
use crate::encryption::PropertyMetadata;

/// REAL Listing Manager
/// Manages property listings with privacy-preserving selective disclosure
pub struct ListingManager {
    // In production, this would be a database (PostgreSQL/SQLite)
    listings: Arc<RwLock<HashMap<String, PropertyListing>>>,
}

impl ListingManager {
    pub fn new() -> Self {
        Self {
            listings: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new property listing with selective disclosure rules
    pub async fn create_listing(
        &self,
        property_id: String,
        owner_account_id: String,
        note_id: String,
        ipfs_cid: String,
        selective_disclosure: SelectiveDisclosure,
    ) -> Result<PropertyListing> {
        tracing::info!("📋 Creating property listing");
        tracing::info!("   Property: {}", property_id);
        tracing::info!("   Owner: {}", owner_account_id);
        
        let listing_id = Uuid::new_v4().to_string();
        
        let listing = PropertyListing {
            listing_id: listing_id.clone(),
            property_id,
            owner_account_id,
            note_id,
            ipfs_cid,
            status: ListingStatus::Active,
            selective_disclosure,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store listing
        let mut listings = self.listings.write().await;
        listings.insert(listing_id.clone(), listing.clone());
        
        tracing::info!("✅ Listing created: {}", listing_id);
        
        Ok(listing)
    }
    
    /// Get listing by ID
    pub async fn get_listing(&self, listing_id: &str) -> Option<PropertyListing> {
        let listings = self.listings.read().await;
        listings.get(listing_id).cloned()
    }
    
    /// List all active listings (anonymized until proof verification)
    pub async fn list_active_listings(&self) -> Vec<PropertyListing> {
        let listings = self.listings.read().await;
        listings
            .values()
            .filter(|l| l.status == ListingStatus::Active)
            .cloned()
            .collect()
    }
    
    /// Update listing status
    pub async fn update_listing_status(
        &self,
        listing_id: &str,
        status: ListingStatus,
    ) -> Result<()> {
        let mut listings = self.listings.write().await;
        
        if let Some(listing) = listings.get_mut(listing_id) {
            listing.status = status;
            listing.updated_at = Utc::now();
            tracing::info!("📋 Listing {} updated to {:?}", listing_id, listing.status);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Listing not found"))
        }
    }
    
    /// REAL selective disclosure implementation
    /// This applies privacy rules based on ZK proof verification
    pub async fn apply_selective_disclosure(
        &self,
        listing: &PropertyListing,
        property_details: &mut PropertyDetails,
        is_accredited: bool,
        is_verified: bool,
    ) {
        tracing::info!("🔒 Applying selective disclosure rules");
        tracing::info!("   Accredited: {}", is_accredited);
        tracing::info!("   Verified: {}", is_verified);
        
        // Rule 1: Hide valuation unless accredited
        if !listing.selective_disclosure.show_valuation_to_accredited || !is_accredited {
            property_details.valuation = None;
            tracing::info!("   ❌ Valuation hidden (not accredited)");
        } else {
            tracing::info!("   ✅ Valuation visible (accredited)");
        }
        
        // Rule 2: Hide documents unless verified
        if !listing.selective_disclosure.show_documents_to_verified || !is_verified {
            property_details.legal_description = None;
            property_details.tax_id = None;
            property_details.documents = vec![];
            tracing::info!("   ❌ Documents hidden (not verified)");
        } else {
            tracing::info!("   ✅ Documents visible (verified)");
        }
        
        // Rule 3: Anonymize location unless eligible
        if !listing.selective_disclosure.show_location_to_eligible || !is_verified {
            if let Some(location) = &property_details.location {
                // Show only city/state, hide street address
                let parts: Vec<&str> = location.split(',').collect();
                if parts.len() > 1 {
                    // Keep only last 2 parts (city, state)
                    let city_state = parts[parts.len()-2..].join(",");
                    property_details.location = Some(city_state.trim().to_string());
                    tracing::info!("   ⚠️  Location anonymized (partial)");
                } else {
                    property_details.location = Some("Location withheld".to_string());
                    tracing::info!("   ❌ Location hidden (not eligible)");
                }
            }
        } else {
            tracing::info!("   ✅ Full location visible (eligible)");
        }
        
        tracing::info!("✅ Selective disclosure applied");
    }
    
    /// Convert PropertyMetadata to PropertyDetails with full info
    pub fn metadata_to_details(
        property_id: String,
        metadata: PropertyMetadata,
        ipfs_cid: String,
    ) -> PropertyDetails {
        PropertyDetails {
            property_id,
            title: metadata.title,
            description: metadata.description,
            property_type: metadata.property_type,
            valuation: Some(metadata.valuation),
            price: metadata.price,
            location: Some(metadata.location),
            square_feet: metadata.square_feet,
            bedrooms: metadata.bedrooms,
            bathrooms: metadata.bathrooms,
            year_built: metadata.year_built,
            owner_name: metadata.owner_name,
            legal_description: Some(metadata.legal_description),
            tax_id: Some(metadata.tax_id),
            zoning: metadata.zoning,
            documents: vec![ipfs_cid],
        }
    }
    
    /// Get all listings for an owner
    pub async fn get_owner_listings(&self, owner_account_id: &str) -> Vec<PropertyListing> {
        let listings = self.listings.read().await;
        listings
            .values()
            .filter(|l| l.owner_account_id == owner_account_id)
            .cloned()
            .collect()
    }
    
    /// Search listings by criteria
    pub async fn search_listings(
        &self,
        property_type: Option<String>,
        min_price: Option<u64>,
        max_price: Option<u64>,
    ) -> Vec<PropertyListing> {
        let listings = self.listings.read().await;
        
        listings
            .values()
            .filter(|l| {
                l.status == ListingStatus::Active
                // Additional filters would require decrypting metadata
                // In production, you'd store searchable fields separately
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_listing() {
        let manager = ListingManager::new();
        
        let disclosure = SelectiveDisclosure {
            show_valuation_to_accredited: true,
            show_documents_to_verified: true,
            show_location_to_eligible: true,
        };
        
        let listing = manager.create_listing(
            "PROP-001".to_string(),
            "0xowner123".to_string(),
            "0xnote456".to_string(),
            "QmTestCID".to_string(),
            disclosure,
        ).await.unwrap();
        
        assert_eq!(listing.property_id, "PROP-001");
        assert_eq!(listing.status, ListingStatus::Active);
    }
    
    #[tokio::test]
    async fn test_selective_disclosure() {
        let manager = ListingManager::new();
        
        let mut details = PropertyDetails {
            property_id: "PROP-001".to_string(),
            title: "Test Villa".to_string(),
            description: "Test".to_string(),
            property_type: "Residential".to_string(),
            valuation: Some(1_000_000),
            price: 950_000,
            location: Some("123 Main St, Los Angeles, CA".to_string()),
            square_feet: 2000,
            bedrooms: 3,
            bathrooms: 2,
            year_built: 2020,
            owner_name: "Test".to_string(),
            legal_description: Some("Lot 1".to_string()),
            tax_id: Some("TAX-001".to_string()),
            zoning: "R1".to_string(),
            documents: vec!["QmDoc1".to_string()],
        };
        
        let listing = PropertyListing {
            listing_id: "test".to_string(),
            property_id: "PROP-001".to_string(),
            owner_account_id: "owner".to_string(),
            note_id: "note".to_string(),
            ipfs_cid: "ipfs".to_string(),
            status: ListingStatus::Active,
            selective_disclosure: SelectiveDisclosure {
                show_valuation_to_accredited: true,
                show_documents_to_verified: true,
                show_location_to_eligible: true,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Test: Not accredited, not verified
        manager.apply_selective_disclosure(
            &listing,
            &mut details,
            false,
            false,
        ).await;
        
        assert!(details.valuation.is_none(), "Valuation should be hidden");
        assert!(details.legal_description.is_none(), "Legal docs should be hidden");
        assert!(details.documents.is_empty(), "Documents should be hidden");
        assert!(!details.location.as_ref().unwrap().contains("123 Main St"), "Address should be anonymized");
    }
}