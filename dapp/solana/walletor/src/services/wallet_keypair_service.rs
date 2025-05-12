use anyhow::Result;
use sea_orm::DbConn;
use solana_sdk::signature::Keypair;
use solana_seed_derivable::SeedDerivable;
use uuid::Uuid;

use crate::crypto::{decrypt_aes_gcm, derive_key_from_passcode, join_secret_xor};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};

pub async fn reconstruct_wallet_keypair(db: &DbConn, user_id: Uuid) -> Result<Keypair> {
    let wallet = wallet_repository::find_wallet_by_user_id(db, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

    let wallet_share = wallet_share_repository::find_share_by_wallet_id(db, wallet.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet share not found"))?;

    let encryption_secret = encryption_secret_repository::find_secret_by_user_id(db, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Encryption secret not found"))?;

    let passcode = "1981"; // ✅ Hardcoded now; feel free to make configurable later
    let derived_key = derive_key_from_passcode(passcode, &encryption_secret.salt)?;

    let decrypted_encryption_secret =
        decrypt_aes_gcm(&derived_key, &encryption_secret.encrypted_secret)?;

    let user_share = decrypt_aes_gcm(
        &decrypted_encryption_secret,
        &wallet_share.encrypted_user_share,
    )?;

    let secret_bytes = join_secret_xor(&user_share, &wallet_share.server_share);

    let keypair = Keypair::from_seed(&secret_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to reconstruct wallet keypair: {}", e))?;

    Ok(keypair)
}
