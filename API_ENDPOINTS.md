# Miden Property Platform - API Endpoints

Complete REST API documentation for the 19-step user journey.

**Base URL:** `http://127.0.0.1:3000`

---

## Table of Contents

1. [Alice (Seller) Endpoints](#alice-seller-endpoints) - Steps 1-6
2. [Bob (Investor) Endpoints](#bob-investor-endpoints) - Steps 7-13
3. [Platform Verification Endpoints](#platform-verification-endpoints) - Steps 14-18
4. [Proof Dashboard Endpoints](#proof-dashboard-endpoints) - Step 19

---

## Alice (Seller) Endpoints

### Step 1: Connect Wallet

**POST** `/api/v1/alice/connect-wallet`

Connect Alice's wallet to the platform.

**Request:**
```json
{}
```

**Response:**
```json
{
  "success": true,
  "wallet": {
    "account_id": "0x24e4b0c8...",
    "account_type": "seller",
    "is_connected": true
  },
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/alice/connect-wallet \
  -H "Content-Type: application/json"
```

---

### Step 2: Mint Property NFT

**POST** `/api/v1/alice/mint-property`

Mint a property as a private NFT with encrypted metadata.

**Request:**
```json
{
  "property_id": "PROP-001",
  "title": "Luxury Beach Villa",
  "description": "Stunning oceanfront property with panoramic views",
  "property_type": "Residential",
  "valuation": 2500000,
  "price": 2300000,
  "location": "123 Ocean Drive, Malibu, CA 90265",
  "square_feet": 4500,
  "bedrooms": 5,
  "bathrooms": 4,
  "year_built": 2018,
  "owner_name": "Alice Johnson",
  "legal_description": "Lot 42, Block 7, Seaside Estates",
  "tax_id": "TAX-2024-001",
  "zoning": "R1"
}
```

**Response:**
```json
{
  "success": true,
  "transaction_id": "0xabc123...",
  "note_id": "0xdef456...",
  "ipfs_cid": "QmXYZ789...",
  "property_id": "PROP-001",
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/alice/mint-property \
  -H "Content-Type: application/json" \
  -d '{
    "property_id": "PROP-001",
    "title": "Luxury Beach Villa",
    "description": "Stunning oceanfront property",
    "property_type": "Residential",
    "valuation": 2500000,
    "price": 2300000,
    "location": "123 Ocean Drive, Malibu, CA",
    "square_feet": 4500,
    "bedrooms": 5,
    "bathrooms": 4,
    "year_built": 2018,
    "owner_name": "Alice Johnson",
    "legal_description": "Lot 42, Block 7",
    "tax_id": "TAX-2024-001",
    "zoning": "R1"
  }'
```

---

### Step 3: View My Property

**GET** `/api/v1/alice/view-property/:ipfs_cid`

View decrypted property metadata (owner only).

**Response:**
```json
{
  "success": true,
  "metadata": {
    "title": "Luxury Beach Villa",
    "description": "Stunning oceanfront property",
    "property_type": "Residential",
    "valuation": 2500000,
    "price": 2300000,
    "location": "123 Ocean Drive, Malibu, CA",
    "square_feet": 4500,
    "bedrooms": 5,
    "bathrooms": 4,
    "year_built": 2018,
    "owner_name": "Alice Johnson",
    "legal_description": "Lot 42, Block 7",
    "tax_id": "TAX-2024-001",
    "zoning": "R1"
  },
  "error": null
}
```

**cURL Example:**
```bash
curl http://127.0.0.1:3000/api/v1/alice/view-property/QmXYZ789...
```

---

### Step 4: List Property for Sale

**POST** `/api/v1/alice/list-property`

List property with selective disclosure rules.

**Request:**
```json
{
  "property_id": "PROP-001",
  "note_id": "0xdef456...",
  "ipfs_cid": "QmXYZ789...",
  "show_valuation_to_accredited": true,
  "show_documents_to_verified": true,
  "show_location_to_eligible": true
}
```

**Response:**
```json
{
  "success": true,
  "listing": {
    "listing_id": "uuid-123",
    "property_id": "PROP-001",
    "owner_account_id": "0x24e4b0c8...",
    "note_id": "0xdef456...",
    "ipfs_cid": "QmXYZ789...",
    "status": "Active",
    "selective_disclosure": {
      "show_valuation_to_accredited": true,
      "show_documents_to_verified": true,
      "show_location_to_eligible": true
    },
    "created_at": "2025-01-15T10:30:00Z",
    "updated_at": "2025-01-15T10:30:00Z"
  },
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/alice/list-property \
  -H "Content-Type: application/json" \
  -d '{
    "property_id": "PROP-001",
    "note_id": "0xdef456...",
    "ipfs_cid": "QmXYZ789...",
    "show_valuation_to_accredited": true,
    "show_documents_to_verified": true,
    "show_location_to_eligible": true
  }'
```

---

### Step 5a: Approve Offer

**POST** `/api/v1/alice/approve-offer`

Approve a purchase offer from investor.

**Request:**
```json
{
  "offer_id": "offer-uuid-456"
}
```

**Response:**
```json
{
  "success": true,
  "offer": {
    "offer_id": "offer-uuid-456",
    "listing_id": "uuid-123",
    "buyer_account_id": "0x98765...",
    "seller_account_id": "0x24e4b0c8...",
    "offer_amount": 2300000,
    "status": "Accepted",
    "escrow_account_id": null,
    "created_at": "2025-01-15T11:00:00Z",
    "updated_at": "2025-01-15T11:15:00Z"
  },
  "error": null
}
```

---

### Step 5b: Reject Offer

**POST** `/api/v1/alice/reject-offer`

Reject a purchase offer.

**Request:**
```json
{
  "offer_id": "offer-uuid-456"
}
```

**Response:** Same structure as approve, with `"status": "Rejected"`

---

### Step 6: Confirm Settlement

**POST** `/api/v1/alice/confirm-settlement/:settlement_id`

Confirm final settlement after atomic execution.

**Response:**
```json
{
  "success": true,
  "settlement": {
    "settlement_id": "settlement-uuid-789",
    "offer_id": "offer-uuid-456",
    "property_note_id": "0xdef456...",
    "escrow_account_id": "0xescrow123...",
    "funds_transfer_tx": "0xfunds-tx...",
    "ownership_transfer_tx": "0xowner-tx...",
    "status": "Completed",
    "created_at": "2025-01-15T12:00:00Z",
    "completed_at": "2025-01-15T12:10:00Z"
  },
  "error": null
}
```

---

## Bob (Investor) Endpoints

### Step 7: Connect Wallet

**POST** `/api/v1/bob/connect-wallet`

Connect Bob's wallet to the platform.

**Request:**
```json
{}
```

**Response:**
```json
{
  "success": true,
  "wallet": {
    "account_id": "0x98765...",
    "account_type": "investor",
    "is_connected": true
  },
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/bob/connect-wallet \
  -H "Content-Type: application/json"
```

---

### Step 8: View Available Listings

**GET** `/api/v1/bob/view-listings`

View all active property listings (anonymized until eligible).

**Response:**
```json
{
  "success": true,
  "listings": [
    {
      "listing_id": "uuid-123",
      "property_id": "PROP-001",
      "owner_account_id": "0x24e4b0c8...",
      "note_id": "0xdef456...",
      "ipfs_cid": "QmXYZ789...",
      "status": "Active",
      "selective_disclosure": {
        "show_valuation_to_accredited": true,
        "show_documents_to_verified": true,
        "show_location_to_eligible": true
      },
      "created_at": "2025-01-15T10:30:00Z",
      "updated_at": "2025-01-15T10:30:00Z"
    }
  ],
  "error": null
}
```

**cURL Example:**
```bash
curl http://127.0.0.1:3000/api/v1/bob/view-listings
```

---

### Step 9: Generate Accreditation Proof

**POST** `/api/v1/bob/generate-accreditation-proof`

Generate ZK proof of accreditation status (CLIENT-SIDE).

**Request:**
```json
{
  "net_worth": 5000000,
  "threshold": 1000000
}
```

**Response:**
```json
{
  "success": true,
  "proof": {
    "proof_bytes": [/* binary data */],
    "program_hash": "0xprog123...",
    "public_inputs": [1000000],
    "public_outputs": [1],
    "proof_type": "accreditation-stark-v1",
    "timestamp": 1705320000
  },
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/bob/generate-accreditation-proof \
  -H "Content-Type: application/json" \
  -d '{
    "net_worth": 5000000,
    "threshold": 1000000
  }'
```

---

### Step 10: Generate Jurisdiction Proof

**POST** `/api/v1/bob/generate-jurisdiction-proof`

Generate ZK proof of jurisdiction eligibility (CLIENT-SIDE).

**Request:**
```json
{
  "country_code": "CA",
  "restricted_countries": ["US", "IR", "KP"]
}
```

**Response:**
```json
{
  "success": true,
  "proof": {
    "proof_bytes": [/* binary data */],
    "program_hash": "0xjuris456...",
    "public_inputs": [3],
    "public_outputs": [1],
    "proof_type": "jurisdiction-stark-v1",
    "timestamp": 1705320100
  },
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/bob/generate-jurisdiction-proof \
  -H "Content-Type: application/json" \
  -d '{
    "country_code": "CA",
    "restricted_countries": ["US", "IR", "KP"]
  }'
```

---

### Step 11: Unlock Property Details

**POST** `/api/v1/bob/unlock-property-details`

Unlock full property details after proof verification.

**Request:**
```json
{
  "listing_id": "uuid-123",
  "accreditation_proof": {
    "proof_bytes": [/* ... */],
    "program_hash": "0xprog123...",
    "public_inputs": [1000000],
    "public_outputs": [1],
    "proof_type": "accreditation-stark-v1",
    "timestamp": 1705320000
  },
  "jurisdiction_proof": {
    "proof_bytes": [/* ... */],
    "program_hash": "0xjuris456...",
    "public_inputs": [3],
    "public_outputs": [1],
    "proof_type": "jurisdiction-stark-v1",
    "timestamp": 1705320100
  }
}
```

**Response:**
```json
{
  "success": true,
  "details": {
    "property_id": "PROP-001",
    "title": "Luxury Beach Villa",
    "description": "Stunning oceanfront property",
    "property_type": "Residential",
    "valuation": 2500000,
    "price": 2300000,
    "location": "123 Ocean Drive, Malibu, CA",
    "square_feet": 4500,
    "bedrooms": 5,
    "bathrooms": 4,
    "year_built": 2018,
    "owner_name": "Alice Johnson",
    "legal_description": "Lot 42, Block 7",
    "tax_id": "TAX-2024-001",
    "zoning": "R1",
    "documents": ["QmXYZ789..."]
  },
  "error": null
}
```

**Note:** If proofs are invalid, sensitive fields will be `null`.

---

### Step 12: Submit Purchase Offer

**POST** `/api/v1/bob/submit-offer`

Submit a purchase offer for a property.

**Request:**
```json
{
  "listing_id": "uuid-123",
  "buyer_account_id": "0x98765...",
  "offer_amount": 2300000
}
```

**Response:**
```json
{
  "success": true,
  "offer": {
    "offer_id": "offer-uuid-456",
    "listing_id": "uuid-123",
    "buyer_account_id": "0x98765...",
    "seller_account_id": "0x24e4b0c8...",
    "offer_amount": 2300000,
    "status": "Pending",
    "escrow_account_id": null,
    "created_at": "2025-01-15T11:00:00Z",
    "updated_at": "2025-01-15T11:00:00Z"
  },
  "error": null
}
```

---

### Step 13: Lock Funds in Escrow

**POST** `/api/v1/bob/lock-funds`

Lock funds in escrow after offer acceptance.

**Request:**
```json
{
  "offer_id": "offer-uuid-456"
}
```

**Response:**
```json
{
  "success": true,
  "transaction_id": "0xescrow-fund-tx...",
  "escrow_account_id": "0xescrow123...",
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/bob/lock-funds \
  -H "Content-Type: application/json" \
  -d '{
    "offer_id": "offer-uuid-456"
  }'
```

---

## Platform Verification Endpoints

### Step 14: Verify Accreditation Proof

**POST** `/api/v1/platform/verify-accreditation-proof`

Platform verifies accreditation proof without seeing private data.

**Request:**
```json
{
  "proof": {
    "proof_bytes": [/* ... */],
    "program_hash": "0xprog123...",
    "public_inputs": [1000000],
    "public_outputs": [1],
    "proof_type": "accreditation-stark-v1",
    "timestamp": 1705320000
  }
}
```

**Response:**
```json
{
  "success": true,
  "valid": true,
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/platform/verify-accreditation-proof \
  -H "Content-Type: application/json" \
  -d @accreditation_proof.json
```

---

### Step 15: Verify Jurisdiction Proof

**POST** `/api/v1/platform/verify-jurisdiction-proof`

Platform verifies jurisdiction proof without seeing user's country.

**Request:**
```json
{
  "proof": {
    "proof_bytes": [/* ... */],
    "program_hash": "0xjuris456...",
    "public_inputs": [3],
    "public_outputs": [1],
    "proof_type": "jurisdiction-stark-v1",
    "timestamp": 1705320100
  }
}
```

**Response:**
```json
{
  "success": true,
  "valid": true,
  "error": null
}
```

---

### Step 16: Verify Ownership Before Mint

**POST** `/api/v1/platform/verify-ownership`

Verify ownership proof before allowing property minting.

**Request:**
```json
{
  "property_id": "PROP-001",
  "document_hash": "0xdoc123abc..."
}
```

**Response:**
```json
{
  "success": true,
  "valid": true,
  "error": null
}
```

**cURL Example:**
```bash
curl -X POST http://127.0.0.1:3000/api/v1/platform/verify-ownership \
  -H "Content-Type: application/json" \
  -d '{
    "property_id": "PROP-001",
    "document_hash": "0xdoc123abc..."
  }'
```

---

### Step 17: Verify Compliance Before Settlement

**GET** `/api/v1/platform/verify-compliance/:offer_id`

Verify all compliance requirements are met before settlement.

**Response:**
```json
{
  "success": true,
  "valid": true,
  "error": null
}
```

**cURL Example:**
```bash
curl http://127.0.0.1:3000/api/v1/platform/verify-compliance/offer-uuid-456
```

---

### Step 18: Execute Atomic Settlement

**POST** `/api/v1/platform/execute-settlement`

Execute atomic settlement (funds + ownership transfer).

**Request:**
```json
{
  "settlement_id": "settlement-uuid-789"
}
```

**Response:**
```json
{
  "success": true,
  "settlement": {
    "settlement_id": "settlement-uuid-789",
    "offer_id": "offer-uuid-456",
    "property_note_id": "0xdef456...",
    "escrow_account_id": "0xescrow123...",
    "funds_transfer_tx": "0xfunds-tx...",
    "ownership_transfer_tx": "0xowner-tx...",
    "status": "Completed",
    "created_at": "2025-01-15T12:00:00Z",
    "completed_at": "2025-01-15T12:10:00Z"
  },
  "error": null
}
```

---

## Proof Dashboard Endpoints

### Step 19a: Get All Proof Events

**GET** `/api/v1/dashboard/proof-events`

View all proof generation and verification events.

**Response:**
```json
{
  "success": true,
  "events": [
    {
      "event_id": "event-uuid-001",
      "account_id": "0x98765...",
      "proof_type": "accreditation",
      "status": "Generated",
      "program_hash": "0xprog123...",
      "created_at": "2025-01-15T11:05:00Z"
    },
    {
      "event_id": "event-uuid-002",
      "account_id": "platform",
      "proof_type": "accreditation",
      "status": "Verified",
      "program_hash": "0xprog123...",
      "created_at": "2025-01-15T11:06:00Z"
    }
  ],
  "error": null
}
```

**cURL Example:**
```bash
curl http://127.0.0.1:3000/api/v1/dashboard/proof-events
```

---

### Step 19b: Get Proof History for Account

**GET** `/api/v1/dashboard/proof-history/:account_id`

View proof history for a specific account.

**Response:**
```json
{
  "success": true,
  "events": [
    {
      "event_id": "event-uuid-001",
      "account_id": "0x98765...",
      "proof_type": "accreditation",
      "status": "Generated",
      "program_hash": "0xprog123...",
      "created_at": "2025-01-15T11:05:00Z"
    },
    {
      "event_id": "event-uuid-003",
      "account_id": "0x98765...",
      "proof_type": "jurisdiction",
      "status": "Generated",
      "program_hash": "0xjuris456...",
      "created_at": "2025-01-15T11:07:00Z"
    }
  ],
  "error": null
}
```

**cURL Example:**
```bash
curl http://127.0.0.1:3000/api/v1/dashboard/proof-history/0x98765...
```

---

## Health Check

**GET** `/health`

Check if the server is running.

**Response:**
```json
{
  "status": "healthy",
  "service": "miden-property-platform",
  "version": "1.0.0",
  "features": [
    "private-nft-minting",
    "zk-proofs",
    "selective-disclosure",
    "atomic-settlement",
    "escrow"
  ]
}
```

---

## Error Responses

All endpoints return errors in the following format:
```json
{
  "success": false,
  "error": "Error message describing what went wrong"
}
```

Common HTTP status codes:
- `200 OK` - Success
- `400 Bad Request` - Invalid input
- `500 Internal Server Error` - Server error

---

## Authentication

Currently, the API does not require authentication. In production, implement:
- JWT tokens
- Account signature verification
- Rate limiting

---

## Rate Limiting

No rate limiting in current version. Implement in production.

---

## Complete Example Workflow

See `examples/complete_workflow.sh` for a full curl-based example of the 19-step journey.