// src/nft.rs - Property NFTs using Fungible Assets (Miden v0.12 Compatible)
use anyhow::Result;
use miden_client::{
    account::AccountId,
    note::{NoteAssets, NoteExecutionHint, NoteInputs, NoteMetadata, NoteRecipient, NoteTag, NoteType},
    ClientRng, Felt, Serializable, Word,
};
use miden_objects::{
    asset::{Asset, FungibleAsset},
    note::{Note, NoteScript},
    FieldElement,
};
use miden_lib::transaction::TransactionKernel;
use sha2::{Digest as Sha2Digest, Sha256};

/// Represents a real property NFT on Miden (using fungible asset with amount=1)
#[derive(Debug, Clone)]
pub struct PropertyNFT {
    pub property_id: String,
    pub ipfs_cid: String,
    pub nft_id: u64,
    pub asset: FungibleAsset,
}

impl PropertyNFT {
    /// Create a new property NFT with unique ID and IPFS CID
    pub fn new(
        faucet_id: AccountId,
        property_id: String,
        ipfs_cid: String,
    ) -> Result<Self> {
        // Generate unique NFT ID from property ID
        let nft_id = Self::generate_nft_id(&property_id);
        
        // Create a fungible asset with amount=1 to represent the unique property NFT
        // This is a common pattern when NonFungibleFaucet is not available
        let asset = FungibleAsset::new(faucet_id, 1)?;

        Ok(Self {
            property_id,
            ipfs_cid,
            nft_id,
            asset,
        })
    }

    /// Generate deterministic NFT ID from property ID
    fn generate_nft_id(property_id: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(property_id.as_bytes());
        let hash = hasher.finalize();
        
        // Take first 8 bytes and convert to u64
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash[..8]);
        u64::from_le_bytes(bytes)
    }

    /// Encode IPFS CID into Felt array for note inputs
    pub fn encode_ipfs_cid(ipfs_cid: &str) -> Result<[Felt; 4]> {
        let mut hasher = Sha256::new();
        hasher.update(ipfs_cid.as_bytes());
        let hash = hasher.finalize();

        // Split hash into 4 Felt values
        let mut felts = [Felt::ZERO; 4];
        for i in 0..4 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&hash[i * 8..(i + 1) * 8]);
            let value = u64::from_le_bytes(bytes);
            felts[i] = Felt::new(value);
        }

        Ok(felts)
    }

    /// Create custom note with embedded IPFS CID and property data
    pub fn create_custom_note(
        &self,
        sender_id: AccountId,
        target_id: AccountId,
        _rng: &mut ClientRng,
    ) -> Result<Note> {
        // Encode IPFS CID into note inputs
        let ipfs_felts = Self::encode_ipfs_cid(&self.ipfs_cid)?;
        
        // Create note inputs: [property_hash, ipfs_part1, ipfs_part2, ipfs_part3, ipfs_part4]
        let mut hasher = Sha256::new();
        hasher.update(self.property_id.as_bytes());
        let prop_hash = hasher.finalize();
        let prop_felt = Felt::new(u64::from_le_bytes(
            prop_hash[..8].try_into().unwrap()
        ));

        let note_inputs = NoteInputs::new(vec![
            prop_felt,
            ipfs_felts[0],
            ipfs_felts[1],
            ipfs_felts[2],
            ipfs_felts[3],
        ])?;

        // Create note assets with the property token (amount=1)
        let note_assets = NoteAssets::new(vec![Asset::Fungible(self.asset)])?;

        // Create note tag
        let note_tag = NoteTag::from_account_id(target_id);

        // Create note metadata
        let note_metadata = NoteMetadata::new(
            sender_id,
            NoteType::Public,
            note_tag,
            NoteExecutionHint::always(),
            Felt::ZERO,
        )?;

        // Use a simple default note script (P2ID-style)
        let note_script = Self::create_default_note_script()?;
        
        // Generate random serial number for recipient
        let serial_num: Word = [
            Felt::new(rand::random::<u64>()),
            Felt::new(rand::random::<u64>()),
            Felt::new(rand::random::<u64>()),
            Felt::new(rand::random::<u64>()),
        ].into();

        // Create note recipient
        let note_recipient = NoteRecipient::new(
            serial_num,
            note_script.clone(),
            note_inputs.clone(),
        );

        // Create the note
        Ok(Note::new(
            note_assets,
            note_metadata,
            note_recipient,
        ))
    }

    /// Create a simple default note script
    fn create_default_note_script() -> Result<NoteScript> {
        // Use the simplest possible P2ID-style script
        let kernel = TransactionKernel::assembler();
        let program = kernel
            .assemble_program(
                "
                begin
                    # Simple P2ID-style transfer
                    # Property metadata is stored in note inputs
                    dropw
                end
                "
            )
            .map_err(|e| anyhow::anyhow!("Failed to compile script: {}", e))?;
        
        Ok(NoteScript::new(program))
    }
}

/// NFT Metadata stored on-chain
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NFTMetadata {
    pub property_id: String,
    pub ipfs_cid: String,
    pub nft_id: u64,
    pub owner: String,
    pub minted_at: i64,
}

impl NFTMetadata {
    pub fn new(property_id: String, ipfs_cid: String, nft_id: u64, owner: String) -> Self {
        Self {
            property_id,
            ipfs_cid,
            nft_id,
            owner,
            minted_at: chrono::Utc::now().timestamp(),
        }
    }
}