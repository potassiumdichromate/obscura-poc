$file = "src/main.rs"
$content = Get-Content $file -Raw

# Pattern to match and fix
$pattern = '(?s)(Ok\(Err\(e\)\) \| Err\(_\) => \(\s+StatusCode::\w+,\s+Json\(\w+ \{\s+success: false,.*?error: Some\(e\),)'

# Replace all occurrences
$content = $content -replace 'Ok\(Err\(e\)\) \| Err\(_\) =>', 'Ok(Err(e)) =>'

# Add Err(_) case after each Ok(Err(e)) case
$handlers = @(
    'ConnectWalletResponse',
    'MintPropertyResponse', 
    'ViewPropertyResponse',
    'ListPropertyResponse',
    'OfferActionResponse',
    'SettlementResponse',
    'ListingsResponse',
    'ZkProofResponse',
    'PropertyDetailsResponse',
    'SubmitOfferResponse',
    'LockFundsResponse',
    'VerifyProofResponse',
    'ProofEventsResponse'
)

foreach ($handler in $handlers) {
    # Find pattern and add Err case
    $pattern = "(\s+Ok\(Err\(e\)\) => \(\s+StatusCode::\w+,\s+Json\($handler \{[^}]+error: Some\(e\),\s+\}\),\s+\))"
    
    $replacement = '$1,
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(' + $handler + ' {
                success: false,
                ' + $(if ($handler -eq 'ConnectWalletResponse') { 'wallet: None,' } 
                     elseif ($handler -eq 'MintPropertyResponse') { 'transaction_id: None, note_id: None, ipfs_cid: None, property_id: None,' }
                     elseif ($handler -eq 'ViewPropertyResponse') { 'metadata: None,' }
                     elseif ($handler -eq 'ListPropertyResponse') { 'listing: None,' }
                     elseif ($handler -eq 'OfferActionResponse') { 'offer: None,' }
                     elseif ($handler -eq 'SettlementResponse') { 'settlement: None,' }
                     elseif ($handler -eq 'ListingsResponse') { 'listings: vec![],' }
                     elseif ($handler -eq 'ZkProofResponse') { 'proof: None,' }
                     elseif ($handler -eq 'PropertyDetailsResponse') { 'details: None,' }
                     elseif ($handler -eq 'SubmitOfferResponse') { 'offer: None,' }
                     elseif ($handler -eq 'LockFundsResponse') { 'transaction_id: None, escrow_account_id: None,' }
                     elseif ($handler -eq 'VerifyProofResponse') { 'valid: false,' }
                     elseif ($handler -eq 'ProofEventsResponse') { 'events: vec![],' }) + '
                error: Some("Client unavailable".to_string()),
            }),
        )'
    
    $content = $content -replace $pattern, $replacement
}

$content | Set-Content $file
Write-Host "✅ Fixed all handlers!" -ForegroundColor Green