use chrono::Utc;
use sea_orm::DbConn;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use uuid::Uuid;

use crate::crypto::{
    derive_key_from_passcode, encrypt_aes_gcm, generate_solana_keypair, split_secret_xor,
};
use crate::models::{encryption_secret, wallet, wallet_share};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};

pub struct CreateWalletResponse {
    pub wallet_id: Uuid,
    pub wallet_address: String,
    pub wallet_public_key: String,
}

pub async fn create_wallet_service(
    db: &DbConn,
    user_id: Uuid,
    passcode: &str,
) -> anyhow::Result<CreateWalletResponse> {
    // 1. Generate Solana Keypair
    let keypair = generate_solana_keypair();
    let private_key_bytes = keypair.secret().as_bytes(); // 32 bytes
    let public_key = keypair.pubkey().to_string();
    let wallet_address = public_key.clone();

    // 2. Split private key into (user_share, server_share)
    let (user_share, server_share) = split_secret_xor(private_key_bytes);

    // 3. Generate random encryption_secret (32 bytes)
    let encryption_secret = crate::crypto::generate_random_bytes(32);

    // 4. Encrypt user_share with encryption_secret
    let encrypted_user_share = encrypt_aes_gcm(&encryption_secret, &user_share)?;

    // 5. Derive key from passcode + random salt
    let salt = crate::crypto::generate_random_bytes(16);
    let derived_key = derive_key_from_passcode(passcode, &salt)?;

    // 6. Encrypt encryption_secret with derived_key
    let encrypted_encryption_secret = encrypt_aes_gcm(&derived_key, &encryption_secret)?;

    // 7. Insert wallet
    let wallet_model =
        wallet_repository::insert_new_wallet(db, user_id, &wallet_address, &public_key).await?;

    // 8. Insert encryption_secret
    let encryption_secret_model = encryption_secret_repository::insert_new_encryption_secret(
        db,
        user_id,
        encrypted_encryption_secret,
        salt,
    )
    .await?;

    // 9. Insert wallet_share
    let _wallet_share_model = wallet_share_repository::insert_new_wallet_share(
        db,
        wallet_model.id,
        encrypted_user_share,
        server_share,
        encryption_secret_model.id,
    )
    .await?;

    Ok(CreateWalletResponse {
        wallet_id: wallet_model.id,
        wallet_address,
        wallet_public_key: public_key,
    })
}
