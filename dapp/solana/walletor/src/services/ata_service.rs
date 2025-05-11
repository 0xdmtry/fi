use crate::config::AppConfig;
use crate::crypto::{decrypt_aes_gcm, join_secret_xor};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};
use std::str::FromStr;

use crate::crypto::derive_key_from_passcode;
use crate::payloads::CreateAtaRequest;
use anyhow::Result;
use sea_orm::DbConn;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use solana_seed_derivable::SeedDerivable;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token;
use uuid::Uuid;

pub struct CreateAtaArgs {
    pub user_id: Uuid,
    pub token_mint: String,
    pub wallet_id: Option<Uuid>,
}

pub struct CreateAtaResult {
    pub ata_address: String,
    pub tx_signature: String,
}

pub async fn create_ata_service(
    db: &DbConn,
    _config: &AppConfig,
    args: CreateAtaRequest,
) -> Result<CreateAtaResult> {
    let wallet = wallet_repository::find_wallet_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;


    let wallet_share = wallet_share_repository::find_share_by_wallet_id(db, wallet.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet share not found"))?;


    let encryption_secret = encryption_secret_repository::find_secret_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Encryption secret not found"))?;

    let passcode = "1981";
    let derived_key = derive_key_from_passcode(passcode, &encryption_secret.salt)?;
    
    let decrypted_encryption_secret =
        decrypt_aes_gcm(&derived_key, &encryption_secret.encrypted_secret)?;

    let user_share = decrypt_aes_gcm(
        &decrypted_encryption_secret,
        &wallet_share.encrypted_user_share,
    )?;

    let secret_bytes = join_secret_xor(&user_share, &wallet_share.server_share);
    let keypair = Keypair::from_seed(&secret_bytes).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let wallet_pubkey = keypair.pubkey();

    let token_mint_pubkey = Pubkey::from_str(&args.token_mint)?;

    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_pubkey,
        &token_mint_pubkey,
    );

    let ix = create_associated_token_account(
        &wallet_pubkey,
        &wallet_pubkey,
        &token_mint_pubkey,
        &spl_token::ID,
    );

    let recent_blockhash = RpcClient::new("https://api.devnet.solana.com".to_string())
        .get_latest_blockhash()
        .await?;

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&wallet_pubkey),
        &[&keypair],
        recent_blockhash,
    );

    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    let signature: Signature = rpc.send_and_confirm_transaction(&tx).await?;

    Ok(CreateAtaResult {
        ata_address: ata.to_string(),
        tx_signature: signature.to_string(),
    })
}
