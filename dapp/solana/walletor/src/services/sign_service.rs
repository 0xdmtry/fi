use crate::config::AppConfig;
use crate::crypto::{decrypt_aes_gcm, join_secret_xor};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};

use crate::crypto::derive_key_from_passcode;
use sea_orm::DbConn;
use solana_sdk::{signature::Keypair, transaction::VersionedTransaction};
use solana_seed_derivable::SeedDerivable;
use uuid::Uuid;

use crate::services::rpc_client;

pub struct SignTransactionArgs {
    pub user_id: Uuid,
    pub wallet_id: Option<Uuid>,
    pub transaction_message_bytes: Vec<u8>, // serialized Message (v0)
}

#[derive(Debug)]
pub struct SignTransactionResult {
    pub signed_transaction_bytes: Vec<u8>,
    pub signature_bytes: Vec<u8>,
    pub wallet_address: String,
    pub wallet_public_key: String,
    pub tx_signature: String,
}

pub async fn sign_transaction_service(
    db: &DbConn,
    _config: &AppConfig,
    args: SignTransactionArgs,
) -> anyhow::Result<SignTransactionResult> {

    // Load wallet
    let wallet = wallet_repository::find_wallet_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found for user"))?;


    // Load wallet share
    let wallet_share = wallet_share_repository::find_share_by_wallet_id(db, wallet.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet share not found"))?;


    // Load encryption secret
    let encryption_secret = encryption_secret_repository::find_secret_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Encryption secret not found"))?;



    // Decrypt encryption_secret
    // let derived_key = vec![0u8; 32]; // No passcode logic for now

    let hardcoded_passcode = "1981";
    let derived_key = derive_key_from_passcode(hardcoded_passcode, &encryption_secret.salt)?;


    let decrypted_encryption_secret =
        decrypt_aes_gcm(&derived_key, &encryption_secret.encrypted_secret)?;


    // Decrypt user share
    let user_share = decrypt_aes_gcm(
        &decrypted_encryption_secret,
        &wallet_share.encrypted_user_share,
    )?;

    // Join shares to recover private key
    let secret_bytes = join_secret_xor(&user_share, &wallet_share.server_share);

    let keypair = Keypair::from_seed(&secret_bytes).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let unsigned_tx: VersionedTransaction = bincode::deserialize(&args.transaction_message_bytes)?;


    ////////////////////////////////////////////////////////

    let required_signer_pubkeys = &unsigned_tx.message.static_account_keys()
        [..unsigned_tx.message.header().num_required_signatures as usize];

    for pubkey in required_signer_pubkeys {
        println!("🔐 Signer required: {}", pubkey);
    }

    ///////////////////////////////////////////////////////

    let keypair_refs = vec![&keypair];
    let tx = VersionedTransaction::try_new(unsigned_tx.message, &keypair_refs)
        .map_err(|e| anyhow::anyhow!("Signing failed: {e}"))?;


    let tx_signature = match rpc_client::submit_transaction_to_devnet(&tx).await {
        Ok(sig) => {
            println!("✅ Transaction submitted! Tx ID: {}", sig);
            sig.to_string()
        }
        Err(err) => {
            eprintln!("❌ Failed to submit transaction: {:?}", err);
            return Err(err); // Or decide how you want to propagate this failure
        }
    };


    let signature_bytes = tx.signatures[0].as_ref().to_vec();

    let signed_transaction_bytes = bincode::serialize(&tx)?;


    let signed_transaction_result = SignTransactionResult {
        signed_transaction_bytes,
        signature_bytes,
        wallet_address: wallet.wallet_address,
        wallet_public_key: wallet.wallet_public_key,
        tx_signature,
    };

    Ok(signed_transaction_result)
}
