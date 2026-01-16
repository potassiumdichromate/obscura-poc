-- migrations/001_init.sql
-- Database schema for Miden Property Platform
-- SQLite compatible

-- Property listings table
CREATE TABLE IF NOT EXISTS property_listings (
    listing_id TEXT PRIMARY KEY,
    property_id TEXT NOT NULL UNIQUE,
    owner_account_id TEXT NOT NULL,
    note_id TEXT NOT NULL,
    ipfs_cid TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Active', 'UnderOffer', 'Sold', 'Cancelled')),
    show_valuation_to_accredited BOOLEAN NOT NULL DEFAULT 0,
    show_documents_to_verified BOOLEAN NOT NULL DEFAULT 0,
    show_location_to_eligible BOOLEAN NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_listings_owner ON property_listings(owner_account_id);
CREATE INDEX idx_listings_status ON property_listings(status);
CREATE INDEX idx_listings_property_id ON property_listings(property_id);

-- Purchase offers table
CREATE TABLE IF NOT EXISTS purchase_offers (
    offer_id TEXT PRIMARY KEY,
    listing_id TEXT NOT NULL,
    buyer_account_id TEXT NOT NULL,
    seller_account_id TEXT NOT NULL,
    offer_amount INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Pending', 'Accepted', 'Rejected', 'EscrowFunded', 'Settled', 'Cancelled')),
    escrow_account_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (listing_id) REFERENCES property_listings(listing_id)
);

CREATE INDEX idx_offers_listing ON purchase_offers(listing_id);
CREATE INDEX idx_offers_buyer ON purchase_offers(buyer_account_id);
CREATE INDEX idx_offers_status ON purchase_offers(status);

-- Settlements table
CREATE TABLE IF NOT EXISTS settlements (
    settlement_id TEXT PRIMARY KEY,
    offer_id TEXT NOT NULL,
    property_note_id TEXT NOT NULL,
    escrow_account_id TEXT NOT NULL,
    funds_transfer_tx TEXT,
    ownership_transfer_tx TEXT,
    status TEXT NOT NULL CHECK(status IN ('Initiated', 'FundsTransferred', 'OwnershipTransferred', 'Completed', 'Failed')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    FOREIGN KEY (offer_id) REFERENCES purchase_offers(offer_id)
);

CREATE INDEX idx_settlements_offer ON settlements(offer_id);
CREATE INDEX idx_settlements_status ON settlements(status);

-- Proof events table (for dashboard)
CREATE TABLE IF NOT EXISTS proof_events (
    event_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    proof_type TEXT NOT NULL CHECK(proof_type IN ('accreditation', 'jurisdiction', 'ownership')),
    status TEXT NOT NULL CHECK(status IN ('Generated', 'Verified', 'Failed')),
    program_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_events_account ON proof_events(account_id);
CREATE INDEX idx_events_type ON proof_events(proof_type);
CREATE INDEX idx_events_created ON proof_events(created_at DESC);

-- Wallet connections table
CREATE TABLE IF NOT EXISTS wallet_connections (
    account_id TEXT PRIMARY KEY,
    account_type TEXT NOT NULL,
    balance INTEGER NOT NULL DEFAULT 0,
    is_connected BOOLEAN NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_wallets_type ON wallet_connections(account_type);
CREATE INDEX idx_wallets_connected ON wallet_connections(is_connected);

-- Trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_listings_timestamp 
AFTER UPDATE ON property_listings
BEGIN
    UPDATE property_listings SET updated_at = CURRENT_TIMESTAMP WHERE listing_id = NEW.listing_id;
END;

CREATE TRIGGER IF NOT EXISTS update_offers_timestamp 
AFTER UPDATE ON purchase_offers
BEGIN
    UPDATE purchase_offers SET updated_at = CURRENT_TIMESTAMP WHERE offer_id = NEW.offer_id;
END;

-- Insert sample data for testing (optional)
-- Uncomment to populate with test data

-- INSERT INTO property_listings (
--     listing_id, property_id, owner_account_id, note_id, ipfs_cid, status,
--     show_valuation_to_accredited, show_documents_to_verified, show_location_to_eligible
-- ) VALUES (
--     'test-listing-001',
--     'PROP-TEST-001',
--     '0xtest_alice_account',
--     '0xtest_note_id',
--     'QmTestCID123',
--     'Active',
--     1,
--     1,
--     1
-- );
