use crate::config::AppConfig;
use crate::crypto::{decrypt_aes_gcm, join_secret_xor};
use crate::repositories::{
    encryption_secret_repository, wallet_repository, wallet_share_repository,
};

use crate::crypto::derive_key_from_passcode;
use sea_orm::DbConn;
use solana_sdk::{
    message::{VersionedMessage, v0::Message},
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use uuid::Uuid;
use solana_seed_derivable::SeedDerivable;
use solana_keypair::keypair_from_seed;

use solana_sdk::{pubkey::Pubkey};
use solana_sdk::signature::{Signature, SignerError};

use std::rc::Rc;
use crate::services::rpc_client;

struct DuplicateSigner<'a> {
    inner: &'a dyn Signer,
}

impl<'a> Signer for DuplicateSigner<'a> {

    fn pubkey(&self) -> Pubkey {
        self.inner.pubkey()
    }

    fn try_pubkey(&self) -> Result<Pubkey, SignerError> {
        todo!()
    }

    fn sign_message(&self, message: &[u8]) -> Signature {
        todo!()
    }

    fn try_sign_message(&self, message: &[u8]) -> std::result::Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        self.inner.try_sign_message(message)
    }

    fn is_interactive(&self) -> bool {
        todo!()
    }
}

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
    println!("sign_payload");

    // 1. Load wallet
    let wallet = wallet_repository::find_wallet_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found for user"))?;

    println!("wallet: {:?}", wallet);

    // 2. Load wallet share
    let wallet_share = wallet_share_repository::find_share_by_wallet_id(db, wallet.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Wallet share not found"))?;

    println!("wallet_share: {:?}", wallet_share);

    // 3. Load encryption secret
    let encryption_secret = encryption_secret_repository::find_secret_by_user_id(db, args.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Encryption secret not found"))?;

    println!("encryption_secret: {:?}", encryption_secret);

    // 4. Decrypt encryption_secret
    // let derived_key = vec![0u8; 32]; // No passcode logic for now

    let hardcoded_passcode = "1981";
    let derived_key = derive_key_from_passcode(hardcoded_passcode, &encryption_secret.salt)?;

    println!("derived_key: {:?}", derived_key);

    println!("1. -------------------------------");

    let decrypted_encryption_secret =
        decrypt_aes_gcm(&derived_key, &encryption_secret.encrypted_secret)?;

    println!("2. -------------------------------");

    println!(
        "decrypted_encryption_secret: {:?}",
        decrypted_encryption_secret
    );

    // 5. Decrypt user share
    let user_share = decrypt_aes_gcm(
        &decrypted_encryption_secret,
        &wallet_share.encrypted_user_share,
    )?;
    println!("user_share: {:?}", user_share);

    // 6. Join shares to recover private key
    let secret_bytes = join_secret_xor(&user_share, &wallet_share.server_share);
    println!("secret_bytes: {:?}", secret_bytes);

    println!("3. -------------------------------");

    let keypair = Keypair::from_seed(&secret_bytes).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("keypair: {:?}", keypair);

    println!("4. -------------------------------");

    let unsigned_tx: VersionedTransaction = bincode::deserialize(&args.transaction_message_bytes)?;

    println!("unsigned_tx: {:?}", unsigned_tx);

    println!("7.0 -------------------------------");

    let keypair_refs = vec![&keypair];
    let tx = VersionedTransaction::try_new(unsigned_tx.message, &keypair_refs)
        .map_err(|e| anyhow::anyhow!("Signing failed: {e}"))?;

    println!("tx: {:?}", tx);


    println!("7. -------------------------------");

    match rpc_client::submit_transaction_to_devnet(&tx).await {
        Ok(signature) => {
            println!("✅ Transaction submitted! Tx ID: {}", signature);
        }
        Err(err) => {
            eprintln!("❌ Failed to submit transaction: {:?}", err);
        }
    }

    println!("7.1. -------------------------------");
    

    let signature_bytes = tx.signatures[0].as_ref().to_vec();
    println!("signature_bytes: {:?}", signature_bytes);

    println!("8. -------------------------------");

    let signed_transaction_bytes = bincode::serialize(&tx)?;

    println!("signed_transaction_bytes: {:?}", signed_transaction_bytes);

    println!("9. -------------------------------");

    Ok(SignTransactionResult {
        signed_transaction_bytes,
        signature_bytes,
        wallet_address: wallet.wallet_address,
        wallet_public_key: wallet.wallet_public_key,
    })
}
