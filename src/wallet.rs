// src/wallet.rs
//
// Wallet connection and management

use anyhow::Result;
use miden_client::account::AccountId;
use crate::models::WalletInfo;
use chrono::Utc;

/// Wallet manager for user accounts
pub struct WalletManager {
    // In-memory wallet connections (in production, use Redis/DB)
    connected_wallets: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, WalletInfo>>>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            connected_wallets: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Connect wallet (register account ID)
    pub async fn connect_wallet(&self, account_id: AccountId, account_type: String) -> Result<WalletInfo> {
        let account_id_str = account_id.to_string();
        
        let wallet_info = WalletInfo {
            account_id: account_id_str.clone(),
            account_type,
            is_connected: true,
            balance: 0,
            created_at: Utc::now(),
        };
        
        let mut wallets = self.connected_wallets.write().await;
        wallets.insert(account_id_str, wallet_info.clone());
        
        tracing::info!("👛 Wallet connected: {}", wallet_info.account_id);
        
        Ok(wallet_info)
    }
    
    /// Disconnect wallet
    pub async fn disconnect_wallet(&self, account_id: &str) -> Result<()> {
        let mut wallets = self.connected_wallets.write().await;
        wallets.remove(account_id);
        
        tracing::info!("👛 Wallet disconnected: {}", account_id);
        Ok(())
    }
    
    /// Check if wallet is connected
    pub async fn is_connected(&self, account_id: &str) -> bool {
        let wallets = self.connected_wallets.read().await;
        wallets.contains_key(account_id)
    }
    
    /// Get wallet info
    pub async fn get_wallet_info(&self, account_id: &str) -> Option<WalletInfo> {
        let wallets = self.connected_wallets.read().await;
        wallets.get(account_id).cloned()
    }
    
    /// List all connected wallets
    pub async fn list_connected_wallets(&self) -> Vec<WalletInfo> {
        let wallets = self.connected_wallets.read().await;
        wallets.values().cloned().collect()
    }
}