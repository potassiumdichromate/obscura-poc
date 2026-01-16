// tests/integration_test.rs
//
// Integration tests for the complete 19-step user journey

use miden_property_platform::{
    MidenClientWrapper,
    encryption::PropertyMetadata,
    models::*,
};

#[tokio::test]
async fn test_complete_user_journey() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_test_writer()
        .init();

    println!("🚀 Starting complete 19-step integration test");

    // Initialize client
    let mut client = MidenClientWrapper::new()
        .await
        .expect("Failed to initialize client");

    println!("✅ Client initialized");

    // =========================================================================
    // ALICE FLOW - Steps 1-6
    // =========================================================================

    // Step 1: Connect Alice's wallet
    println!("\n📍 Step 1: Connecting Alice's wallet");
    let alice_wallet = client.connect_wallet_alice().await.expect("Failed to connect Alice");
    assert!(alice_wallet["is_connected"].as_bool().unwrap());
    println!("✅ Alice connected");

    // Step 2: Mint property NFT
    println!("\n📍 Step 2: Minting property NFT");
    let metadata = PropertyMetadata {
        title: "Test Villa".to_string(),
        description: "Integration test property".to_string(),
        property_type: "Residential".to_string(),
        valuation: 1_000_000,
        price: 950_000,
        location: "Test City, TC 12345".to_string(),
        square_feet: 2500,
        bedrooms: 4,
        bathrooms: 3,
        year_built: 2020,
        owner_name: "Alice Test".to_string(),
        legal_description: "Lot 1, Block 1, Test Subdivision".to_string(),
        tax_id: "TEST-TAX-001".to_string(),
        zoning: "R1".to_string(),
    };

    let (mint_tx, note_id, ipfs_cid) = client
        .mint_property_nft_private("TEST-PROP-001".to_string(), metadata.clone())
        .await
        .expect("Failed to mint property");

    assert!(!mint_tx.is_empty());
    assert!(!note_id.is_empty());
    assert!(!ipfs_cid.is_empty());
    println!("✅ Property minted");
    println!("   TX: {}", mint_tx);
    println!("   Note: {}", note_id);
    println!("   IPFS: {}", ipfs_cid);

    // Step 3: View property metadata
    println!("\n📍 Step 3: Viewing property metadata");
    let viewed_metadata = client
        .view_my_property(&ipfs_cid)
        .await
        .expect("Failed to view property");

    assert_eq!(viewed_metadata.title, "Test Villa");
    assert_eq!(viewed_metadata.valuation, 1_000_000);
    println!("✅ Metadata retrieved and decrypted");

    // Step 4 is tested via listing manager (not directly in client)
    println!("\n📍 Step 4: Listing property (tested via API)");

    // =========================================================================
    // BOB FLOW - Steps 7-13
    // =========================================================================

    // Step 7: Connect Bob's wallet
    println!("\n📍 Step 7: Connecting Bob's wallet");
    let bob_wallet = client.connect_wallet_bob().await.expect("Failed to connect Bob");
    assert!(bob_wallet["is_connected"].as_bool().unwrap());
    println!("✅ Bob connected");

    // Step 8: View listings (tested via API)
    println!("\n📍 Step 8: Viewing listings (tested via API)");

    // Step 9: Generate accreditation proof
    println!("\n📍 Step 9: Generating accreditation ZK proof");
    let accred_proof = client
        .generate_accreditation_proof(5_000_000, 1_000_000)
        .await
        .expect("Failed to generate accreditation proof");

    assert_eq!(accred_proof.proof_type, "accreditation-stark-v1");
    assert_eq!(accred_proof.public_inputs[0], 1_000_000);
    assert!(!accred_proof.proof_bytes.is_empty());
    println!("✅ Accreditation proof generated");
    println!("   Proof size: {} bytes", accred_proof.proof_bytes.len());

    // Step 10: Generate jurisdiction proof
    println!("\n📍 Step 10: Generating jurisdiction ZK proof");
    let juris_proof = client
        .generate_jurisdiction_proof("CA".to_string(), vec!["US".to_string(), "IR".to_string()])
        .await
        .expect("Failed to generate jurisdiction proof");

    assert_eq!(juris_proof.proof_type, "jurisdiction-stark-v1");
    println!("✅ Jurisdiction proof generated");

    // Step 11-13: Tested via API endpoints

    // =========================================================================
    // PLATFORM VERIFICATION - Steps 14-18
    // =========================================================================

    // Step 14: Verify accreditation proof
    println!("\n📍 Step 14: Verifying accreditation proof");
    let accred_valid = client
        .verify_accreditation_proof(&accred_proof)
        .await
        .expect("Failed to verify accreditation proof");

    assert!(accred_valid, "Accreditation proof should be valid");
    println!("✅ Accreditation proof verified");

    // Step 15: Verify jurisdiction proof
    println!("\n📍 Step 15: Verifying jurisdiction proof");
    let juris_valid = client
        .verify_jurisdiction_proof(&juris_proof)
        .await
        .expect("Failed to verify jurisdiction proof");

    assert!(juris_valid, "Jurisdiction proof should be valid");
    println!("✅ Jurisdiction proof verified");

    // Step 16: Verify ownership before mint
    println!("\n📍 Step 16: Verifying ownership before mint");
    let ownership_valid = client
        .verify_ownership_before_mint("TEST-PROP-001", "0xdocument_hash")
        .await
        .expect("Failed to verify ownership");

    assert!(ownership_valid, "Ownership should be valid");
    println!("✅ Ownership verified");

    // Steps 17-19: Tested via API endpoints

    println!("\n🎉 All integration tests passed!");
}

#[tokio::test]
async fn test_accreditation_proof_fails_below_threshold() {
    let mut client = MidenClientWrapper::new()
        .await
        .expect("Failed to initialize client");

    println!("🧪 Testing accreditation proof failure");

    let result = client
        .generate_accreditation_proof(500_000, 1_000_000)
        .await;

    assert!(result.is_err(), "Should fail when net worth < threshold");
    println!("✅ Correctly rejected insufficient net worth");
}

#[tokio::test]
async fn test_jurisdiction_proof_fails_restricted_country() {
    let mut client = MidenClientWrapper::new()
        .await
        .expect("Failed to initialize client");

    println!("🧪 Testing jurisdiction proof failure");

    let result = client
        .generate_jurisdiction_proof(
            "US".to_string(),
            vec!["US".to_string(), "IR".to_string()]
        )
        .await;

    assert!(result.is_err(), "Should fail when country is restricted");
    println!("✅ Correctly rejected restricted country");
}

#[tokio::test]
async fn test_encryption_roundtrip() {
    use miden_property_platform::encryption::{PropertyEncryption, PropertyMetadata};

    println!("🧪 Testing encryption/decryption");

    let seed = [42u8; 32];
    let encryption = PropertyEncryption::new(&seed).expect("Failed to create encryption");

    let metadata = PropertyMetadata {
        title: "Encryption Test".to_string(),
        description: "Testing encryption".to_string(),
        property_type: "Test".to_string(),
        valuation: 100_000,
        price: 90_000,
        location: "Test Location".to_string(),
        square_feet: 1000,
        bedrooms: 2,
        bathrooms: 1,
        year_built: 2020,
        owner_name: "Test Owner".to_string(),
        legal_description: "Test Legal".to_string(),
        tax_id: "TEST-001".to_string(),
        zoning: "R1".to_string(),
    };

    let encrypted = encryption.encrypt_metadata(&metadata).expect("Failed to encrypt");
    let decrypted = encryption.decrypt_metadata(&encrypted).expect("Failed to decrypt");

    assert_eq!(metadata.title, decrypted.title);
    assert_eq!(metadata.valuation, decrypted.valuation);
    assert_eq!(metadata.location, decrypted.location);

    println!("✅ Encryption/decryption successful");
}

#[test]
fn test_nft_metadata_serialization() {
    use miden_property_platform::nft::NFTMetadata;

    println!("🧪 Testing NFT metadata serialization");

    let metadata = NFTMetadata::new(
        "TEST-PROP-001",
        "QmTestCID123",
        "Test Property"
    );

    let inputs = metadata.to_note_inputs();
    assert_eq!(inputs.len(), 13, "Should have 13 field elements");

    let reconstructed = NFTMetadata::from_note_inputs(&inputs)
        .expect("Failed to reconstruct metadata");

    assert_eq!(metadata.property_id, reconstructed.property_id);
    assert_eq!(metadata.timestamp, reconstructed.timestamp);

    println!("✅ NFT metadata serialization successful");
}