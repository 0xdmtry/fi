use walletor::crypto::*;

#[test]
fn test_split_secret_and_recombine() {
    let secret = generate_random_bytes(32);
    let (user_share, server_share) = split_secret_xor(&secret);

    let recombined: Vec<u8> = user_share
        .iter()
        .zip(server_share.iter())
        .map(|(u, s)| u ^ s)
        .collect();

    assert_eq!(
        secret, recombined,
        "Recombined secret does not match original"
    );
}

#[test]
fn test_encrypt_and_decrypt() {
    let key = generate_random_bytes(32);
    let data = b"test message";

    let ciphertext = encrypt_aes_gcm(&key, data).expect("encryption failed");
    let decrypted = decrypt_aes_gcm(&key, &ciphertext).expect("decryption failed");

    assert_eq!(
        data.to_vec(),
        decrypted,
        "Decrypted text does not match original"
    );
}

#[test]
fn test_derive_key_from_passcode() {
    let passcode = "1234";
    let salt = generate_random_bytes(16);

    let derived_key = derive_key_from_passcode(passcode, &salt).expect("key derivation failed");

    assert_eq!(derived_key.len(), 32, "Derived key must be 32 bytes");
}
