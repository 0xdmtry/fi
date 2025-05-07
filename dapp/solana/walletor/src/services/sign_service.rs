use crate::config::AppConfig;
use crate::crypto::{decrypt_aes_gcm, join_secret_xor};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};

use sea_orm::DbConn;
use solana_sdk::{
    message::{VersionedMessage, v0::Message},
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use uuid::Uuid;

pub struct SignTransactionArgs {
    pub user_id: Uuid,
    pub wallet_id: Option<Uuid>,
    pub transaction_message_bytes: Vec<u8>, // serialized Message (v0)
}

pub struct SignTransactionResult {
    pub signed_transaction_bytes: Vec<u8>,
    pub signature_bytes: Vec<u8>,
    pub wallet_address: String,
    pub wallet_public_key: String,
}

pub async fn sign_transaction_service(
    db: &DbConn,
    _config: &AppConfig,
    args: SignTransactionArgs,
) -> anyhow::Result<SignTransactionResult> {
    // 1. Load wallet
    let wallet = wallet_repository::find_wallet_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found for user"))?;

    // 2. Load wallet share
    let wallet_share = wallet_share_repository::find_share_by_wallet_id(db, wallet.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet share not found"))?;

    // 3. Load encryption secret
    let encryption_secret = encryption_secret_repository::find_secret_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Encryption secret not found"))?;

    // 4. Decrypt encryption_secret
    let derived_key = vec![0u8; 32]; // No passcode logic for now
    let decrypted_encryption_secret =
        decrypt_aes_gcm(&derived_key, &encryption_secret.encrypted_secret)?;

    // 5. Decrypt user share
    let user_share = decrypt_aes_gcm(
        &decrypted_encryption_secret,
        &wallet_share.encrypted_user_share,
    )?;

    // 6. Join shares to recover private key
    let secret_bytes = join_secret_xor(&user_share, &wallet_share.server_share);
    let keypair = Keypair::from_bytes(&secret_bytes)?;

    // 7. Deserialize message and wrap into VersionedMessage
    let message: Message = bincode::deserialize(&args.transaction_message_bytes)?;
    let versioned_message = VersionedMessage::V0(message);

    // 8. Build and sign transaction
    let tx = VersionedTransaction::try_new(versioned_message, &[&keypair])?;
    let signature_bytes = tx.signatures[0].as_ref().to_vec();
    let signed_transaction_bytes = bincode::serialize(&tx)?;

    Ok(SignTransactionResult {
        signed_transaction_bytes,
        signature_bytes,
        wallet_address: wallet.wallet_address,
        wallet_public_key: wallet.wallet_public_key,
    })
}
