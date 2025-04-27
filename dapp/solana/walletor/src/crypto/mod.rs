use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use anyhow::{Result, anyhow};
use rand::rngs::OsRng as SecureOsRng;
use rand_core::RngCore;
use scrypt::{Params, scrypt};
use solana_sdk::signature::{Keypair, Signer};

/// Generate a new random Solana Keypair
pub fn generate_solana_keypair() -> Keypair {
    Keypair::new()
}

/// XOR-split a secret into (user_share, server_share)
pub fn split_secret_xor(secret: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut rng = SecureOsRng;
    let mut server_share = vec![0u8; secret.len()];
    rng.fill_bytes(&mut server_share);

    let user_share: Vec<u8> = secret
        .iter()
        .zip(server_share.iter())
        .map(|(s, r)| s ^ r)
        .collect();

    (user_share, server_share)
}

/// Encrypt data with AES-256-GCM
pub fn encrypt_aes_gcm(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(anyhow!("Encryption key must be 32 bytes for AES-256"));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Failed to create AES cipher"))?;

    let mut nonce_bytes = [0u8; 12];
    SecureOsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|_| anyhow!("Encryption failed"))?;

    let mut output = nonce_bytes.to_vec();
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

/// Decrypt data with AES-256-GCM
pub fn decrypt_aes_gcm(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(anyhow!("Decryption key must be 32 bytes for AES-256"));
    }

    if ciphertext.len() < 12 {
        return Err(anyhow!("Ciphertext too short to contain nonce"));
    }

    let (nonce_bytes, ciphertext) = ciphertext.split_at(12);

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Failed to create AES cipher"))?;

    cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| anyhow!("Decryption failed"))
}


/// Derive a strong 32-byte encryption key from passcode + salt using scrypt
pub fn derive_key_from_passcode(passcode: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let mut derived_key = [0u8; 32];
    let params = Params::recommended(); // N=16384, r=8, p=1

    scrypt(passcode.as_bytes(), salt, &params, &mut derived_key)
        .map_err(|_| anyhow!("scrypt key derivation failed"))?;

    Ok(derived_key)
}

/// Generate random bytes of specified length
pub fn generate_random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    SecureOsRng.fill_bytes(&mut buf);
    buf
}