use crate::config::AppConfig;
use crate::payloads::{FundWsolRequest, FundWsolResponse};
use crate::repositories::wallet_repository;
use crate::services::wallet_keypair_service::reconstruct_wallet_keypair;
use anyhow::Result;
use sea_orm::DbConn;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token;
use std::str::FromStr;

pub async fn fund_wsol_ata(
    db: &DbConn,
    config: &AppConfig,
    args: FundWsolRequest, // includes user_id, amount
) -> Result<FundWsolResponse> {
    let user_id = args.user_id;
    let amount_sol = args.amount_sol; // f64

    // Fetch wallet + keypair
    let wallet = wallet_repository::find_wallet_by_user_id(db, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

    let wallet_keypair = reconstruct_wallet_keypair(db, user_id).await?;
    let wallet_pubkey = wallet_keypair.pubkey();

    // Define WSOL mint and derive ATA
    let token_mint_pubkey = Pubkey::from_str("So11111111111111111111111111111111111111112")?;
    let ata = get_associated_token_address(&wallet_pubkey, &token_mint_pubkey);

    // Convert SOL → lamports
    let lamports = ((amount_sol as f64) * LAMPORTS_PER_SOL as f64) as u64;

    // Step 1: transfer SOL into ATA
    let transfer_ix = system_instruction::transfer(&wallet_pubkey, &ata, lamports);

    // Step 2: sync native (makes SPL ledger recognize the balance as WSOL)
    let sync_ix = spl_token::instruction::sync_native(&spl_token::ID, &ata)?;

    // Step 3: assemble + sign + submit transaction
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    let recent_blockhash = rpc.get_latest_blockhash().await?;

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix, sync_ix],
        Some(&wallet_pubkey),
        &[&wallet_keypair],
        recent_blockhash,
    );

    let signature = rpc.send_and_confirm_transaction(&tx).await?;

    Ok(FundWsolResponse {
        ata_address: ata.to_string(),
        tx_signature: signature.to_string(),
    })
}
