use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;

use anyhow::Result;
use bincode;

pub async fn submit_transaction_to_devnet(tx: &VersionedTransaction) -> anyhow::Result<Signature> {
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    let tx_signature = rpc
        .send_transaction(tx)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit transaction: {}", e))?;

    Ok(tx_signature)
}

pub async fn submit_signed_transaction_bytes(tx_bytes: &[u8]) -> Result<String> {
    let tx: VersionedTransaction = bincode::deserialize(tx_bytes)?;
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    let tx_signature = rpc
        .send_transaction(&tx)
        .await
        .map_err(|e| anyhow::anyhow!("RPC submission failed: {e}"))?;

    Ok(tx_signature.to_string())
}
