use libaes::Cipher;
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, SeekFrom, Write, Seek};


fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
  let mut output_key_material: [u8; 32] = [0u8; 32];
  let _ = Argon2::default().hash_password_into(password.as_bytes(), salt, &mut output_key_material);
  output_key_material
}

fn generate_rand() -> [u8; 16] {
  let mut rng: [u8; 16] = [0u8; 16];
  OsRng.fill_bytes(&mut rng);
  rng
}

pub fn encrypt_file(file_path: &str, output_path: &str, password: &str) -> io::Result<()> {
  let salt: [u8; 16] = generate_rand();
  let iv: [u8; 16] = generate_rand();
  let key: [u8; 32] = derive_key(password, &salt);
  let cipher: Cipher = Cipher::new_256(&key);

  let mut input_file: BufReader<File> = BufReader::new(File::open(file_path)?);
  let mut output_file: BufWriter<File> = BufWriter::new(File::create(output_path)?);

  output_file.write_all(&salt)?;

  let mut hasher = Sha256::new();
  let mut buffer = [0; 4096];

  loop {
    let bytes_read = input_file.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }
    hasher.update(&buffer[..bytes_read]);
    let encrypted = cipher.cbc_encrypt(&iv, &buffer[..bytes_read]);
    output_file.write_all(&encrypted)?;
  }

  let hash = hasher.finalize();
  let mut hash_array: [u8; 32] = [0; 32];
  hash_array.copy_from_slice(&hash);

  output_file.write_all(&iv)?;
  output_file.write_all(&hash_array)?;

  Ok(())
}

pub fn decrypt_file(file_path: &str, output_path: &str, password: &str) -> io::Result<()> {
  let mut encryption_file = BufReader::new(File::open(file_path)?);
  let mut output_file = BufWriter::new(File::create(output_path)?);

  let mut salt = [0u8; 16];
  encryption_file.read_exact(&mut salt)?;
  let key = derive_key(password, &salt);

  // Calculate the position where encrypted data ends (excluding IV and hash)
  let file_len = encryption_file.seek(SeekFrom::End(0))?; // Get total file length
  let data_end_pos = file_len - 48; // IV (16 bytes) + Hash (32 bytes)

  encryption_file.seek(SeekFrom::Start(data_end_pos))?;
  let mut iv = [0u8; 16];
  let mut expected_hash = [0u8; 32];
  encryption_file.read_exact(&mut iv)?;
  encryption_file.read_exact(&mut expected_hash)?;

  encryption_file.seek(SeekFrom::Start(16))?; // Skip salt to start reading encrypted data

  let cipher = Cipher::new_256(&key);
  let mut hasher = Sha256::new();
  let mut buffer = [0; 4096];

  while encryption_file.stream_position()? < data_end_pos {
      let bytes_read = encryption_file.read(&mut buffer)?;
      if bytes_read == 0 {
          break;
      }
      let decrypted_chunk = cipher.cbc_decrypt(&iv, &buffer[..bytes_read]);
      hasher.update(&decrypted_chunk);
      output_file.write_all(&decrypted_chunk)?;
  }

  let calculated_hash = hasher.finalize();
  let mut calculated_hash_array: [u8; 32] = [0; 32];
  calculated_hash_array.copy_from_slice(&calculated_hash);

  if calculated_hash_array != expected_hash {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "Decryption failed: hash mismatch"));
  }

  Ok(())
}
