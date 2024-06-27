
use libaes::Cipher;
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
use std::fs;
use std::io;
use std::env;

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut output_key_material = [0u8; 32];
    let _ = Argon2::default().hash_password_into(password.as_bytes(), salt, &mut output_key_material);
    output_key_material
}

fn generate_rand() -> [u8; 16] {
    let mut rng = [0u8; 16];
    OsRng.fill_bytes(&mut rng);
    rng
}

fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
    fs::read(file_path)
}

fn write_file(file_path: &str, data: Vec<u8>) -> io::Result<()> {
    fs::write(file_path, data)
}

fn calculate_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

fn encrypt_file(file_path: &str, output_path: &str, password: &str) {
    let salt = generate_rand();
    let iv = generate_rand();
    let key = derive_key(password, &salt);

    let cipher = Cipher::new_256(&key);
    let plaintext = match read_file(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[ERROR] Error reading file: {}", e);
            return;
        }
    };

    let hash = calculate_hash(&plaintext);

    let mut encrypted = cipher.cbc_encrypt(&iv, &plaintext[..]);
    encrypted.extend_from_slice(&iv);
    encrypted.extend_from_slice(&hash);
    encrypted = [&salt[..], &encrypted[..]].concat();
    if let Err(e) = write_file(output_path, encrypted) {
        eprintln!("[ERROR] Error writing file: {}", e);
    }
}

fn decrypt_file(file_path: &str, output_path: &str, password: &str) {
    let encrypted = match read_file(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[ERROR] Error reading file: {}", e);
            return;
        }
    };
    let (salt, data) = encrypted.split_at(16);
    let (data, plaintext_hash) = data.split_at(data.len() - 32);
    let (encrypted_data, iv) = data.split_at(data.len() - 16);
    let key = derive_key(password, &salt);

    let cipher = Cipher::new_256(&key);
    let decrypted = cipher.cbc_decrypt(iv, &encrypted_data[..]);
    let calculated_hash = calculate_hash(&decrypted);
    
    if calculated_hash != plaintext_hash {
        let ciphertext_hash_hex = calculated_hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        let plaintext_hash_hex = plaintext_hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        eprintln!("[ERROR] Decryption failed: hash mismatch \n\t{:?} != {:?}", ciphertext_hash_hex, plaintext_hash_hex);
        eprintln!("[ERROR] Could be due to incorrect password or file corruption.");
        return;
    }
    if decrypted.is_empty(){
        eprintln!("[ERROR] Error decrypting file");
        return;
    }
    if let Err(e) = write_file(output_path, decrypted) {
        eprintln!("[ERROR] Error writing file: {}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} [d|e] <input_file> <output_file> <password>", args[0]);
        return;
    }

    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let password = &args[4];

    match mode.as_str() {
        "e" => encrypt_file(input_file, output_file, password),
        "d" => decrypt_file(input_file, output_file, password),
        _ => eprintln!("[ERROR] Invalid mode: {}", mode),
    }
}


