// src/encryption.rs - REAL AES-256-GCM encryption

use anyhow::{Result, Context};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce, Key
};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

/// REAL property metadata (encrypted off-chain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMetadata {
    pub property_id: String,
    pub title: String,
    pub description: String,
    pub property_type: String,
    pub valuation: u64,
    pub price: u64,
    pub location: String,
    pub square_feet: u32,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub year_built: u16,
    pub owner_name: String,
    pub legal_description: String,
    pub tax_id: String,
    pub zoning: String,
}

/// Encrypted metadata container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMetadata {
    pub ciphertext: String,  // Base64
    pub nonce: String,       // Base64
    pub version: String,
}

/// REAL encryption using AES-256-GCM
pub struct PropertyEncryption {
    cipher: Aes256Gcm,
}

impl PropertyEncryption {
    /// Create from account hash (32 bytes)
    pub fn from_account_hash(account_hash: &[u8; 32]) -> Result<Self> {
        let mut hasher = Sha256::new();
        hasher.update(b"MIDEN_PROPERTY_KEY_V1");
        hasher.update(account_hash);
        let key_material = hasher.finalize();
        
        let key = Key::<Aes256Gcm>::from_slice(&key_material);
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }
    
    /// Create from seed (32 bytes) - for testing
    pub fn from_seed(seed: &[u8; 32]) -> Result<Self> {
        let key = Key::<Aes256Gcm>::from_slice(seed);
        let cipher = Aes256Gcm::new(key);
        Ok(Self { cipher })
    }
    
    /// REAL encryption (not mock)
    pub fn encrypt(&self, metadata: &PropertyMetadata) -> Result<Vec<u8>> {
        // Serialize metadata to JSON
        let json = serde_json::to_vec(metadata)
            .context("Failed to serialize metadata")?;
        
        tracing::info!("🔒 Encrypting property metadata ({} bytes)", json.len());
        
        // Generate random nonce (12 bytes for AES-GCM)
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt with authentication
        let ciphertext = self.cipher
            .encrypt(&nonce, json.as_ref())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        tracing::info!("✅ Encrypted to {} bytes", ciphertext.len());
        
        // Create encrypted metadata container
        let encrypted = EncryptedMetadata {
            ciphertext: general_purpose::STANDARD.encode(&ciphertext),
            nonce: general_purpose::STANDARD.encode(&nonce),
            version: "v1".to_string(),
        };
        
        // Serialize to JSON bytes for IPFS storage
        let encrypted_json = serde_json::to_vec(&encrypted)
            .context("Failed to serialize encrypted metadata")?;
        
        Ok(encrypted_json)
    }
    
    /// REAL decryption (not mock)
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<PropertyMetadata> {
        tracing::info!("🔓 Decrypting property metadata");
        
        // Parse JSON
        let encrypted: EncryptedMetadata = serde_json::from_slice(encrypted_data)
            .context("Failed to parse encrypted data as JSON")?;
        
        tracing::info!("✅ Parsed encrypted metadata (version: {})", encrypted.version);
        
        // Decode base64 ciphertext
        let ciphertext = general_purpose::STANDARD
            .decode(&encrypted.ciphertext)
            .context("Invalid ciphertext base64")?;
        
        // Decode base64 nonce
        let nonce_bytes = general_purpose::STANDARD
            .decode(&encrypted.nonce)
            .context("Invalid nonce base64")?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Decrypt with authentication verification
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| anyhow::anyhow!("Decryption failed - wrong key or corrupted data"))?;
        
        tracing::info!("✅ Decrypted {} bytes", plaintext.len());
        
        // Deserialize metadata
        let metadata: PropertyMetadata = serde_json::from_slice(&plaintext)
            .context("Failed to deserialize metadata")?;
        
        tracing::info!("✅ Successfully decrypted property: {}", metadata.property_id);
        
        Ok(metadata)
    }
}

/// Derive encryption key from Miden account ID
pub fn derive_key_from_account_id(account_id: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"MIDEN_ACCOUNT_KEY_DERIVATION_V1");
    hasher.update(account_id.as_bytes());
    
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_real_encryption_roundtrip() {
        let seed = [42u8; 32];
        let enc = PropertyEncryption::from_seed(&seed).unwrap();
        
        let metadata = PropertyMetadata {
            property_id: "TEST-001".to_string(),
            title: "Test Villa".to_string(),
            description: "Luxury property".to_string(),
            property_type: "Residential".to_string(),
            valuation: 1_000_000,
            price: 950_000,
            location: "Test City".to_string(),
            square_feet: 2000,
            bedrooms: 3,
            bathrooms: 2,
            year_built: 2020,
            owner_name: "Test Owner".to_string(),
            legal_description: "Lot 1".to_string(),
            tax_id: "TAX-001".to_string(),
            zoning: "R1".to_string(),
        };
        
        let encrypted = enc.encrypt(&metadata).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();
        
        assert_eq!(metadata.title, decrypted.title);
        assert_eq!(metadata.valuation, decrypted.valuation);
        assert_eq!(metadata.property_id, decrypted.property_id);
    }
    
    #[test]
    fn test_wrong_key_fails() {
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        
        let enc1 = PropertyEncryption::from_seed(&seed1).unwrap();
        let enc2 = PropertyEncryption::from_seed(&seed2).unwrap();
        
        let metadata = PropertyMetadata {
            property_id: "TEST-002".to_string(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            property_type: "Test".to_string(),
            valuation: 100000,
            price: 90000,
            location: "Test".to_string(),
            square_feet: 1000,
            bedrooms: 2,
            bathrooms: 1,
            year_built: 2020,
            owner_name: "Test".to_string(),
            legal_description: "Test".to_string(),
            tax_id: "TEST".to_string(),
            zoning: "R1".to_string(),
        };
        
        let encrypted = enc1.encrypt(&metadata).unwrap();
        
        // Decryption with wrong key should fail
        let result = enc2.decrypt(&encrypted);
        assert!(result.is_err(), "Decryption with wrong key should fail");
    }
    
    #[test]
    fn test_key_derivation() {
        let account_id = "0x1234567890abcdef";
        let key1 = derive_key_from_account_id(account_id);
        let key2 = derive_key_from_account_id(account_id);
        
        assert_eq!(key1, key2, "Same account ID should produce same key");
        
        let different_key = derive_key_from_account_id("0xdifferent");
        assert_ne!(key1, different_key, "Different account IDs should produce different keys");
    }
}