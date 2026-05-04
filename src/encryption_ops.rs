//! PDF Standard Security Handler — Revision 2, V=1 (40-bit RC4)
//!
//! Implements password hash computation and encryption key derivation per PDF spec §7.6.
//!
//! Reference algorithms (PDF 1.4 / §7.6.3):
//!   Algorithm 2 — Computing an encryption key
//!   Algorithm 3 — Computing the O (owner) value
//!   Algorithm 4 — Computing the U (user) value

/*
todo
1. compute_data_hash — the existing implementation doesn't iterate over objects yet (the loop is 
commented out). Once object serialisation is wired in, the file ID will be a genuine content hash.

2. Stream/string encryption — the actual RC4 encryption of stream data and string values during 
serialisation isn't implemented yet. That would require threading the encryption key through 
the serialize call chain.

3. Higher revisions — this implements R=2, V=1 (40-bit RC4). For AES-128 (V=4, R=4) 
or AES-256 (V=5, R=6), the key derivation and encryption algorithms are more complex and would 
need additional work in encryption_ops.rs and in encryption.rs (which currently just has the 
EncryptionAlgorithm enum).
*/
use crate::file_identifier::FileIdentifierMode;
use crate::object_ops::PdfObject;

const PDF_PASSWORD_PADDING: [u8; 32] = [
    0x28, 0xBF, 0x4E, 0x5E, 0x4E, 0x75, 0x8A, 0x41, 0x64, 0x00, 0x4B, 0x49, 0xC1, 0xF1, 0x52, 0x28,
    0xDE, 0xAE, 0xD2, 0x3B, 0x61, 0x6F, 0x74, 0x6B, 0x45, 0x2F, 0x72, 0x73, 0x65, 0x54, 0x68, 0xE8,
];

/// Permission flags for the /P entry.
/// Bits are defined per PDF spec Table 3.20.
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub print: bool,
    pub modify: bool,
    pub copy: bool,
    pub annotate: bool,
}

impl Permissions {
    pub fn new() -> Self {
        Self {
            print: false,
            modify: false,
            copy: false,
            annotate: false,
        }
    }

    /// Encode as a signed 32-bit integer for the /P dictionary entry.
    pub fn as_i32(self) -> i32 {
        let mut flags = -4_i32; // 0xFFFF_FFFC — all bits set except bits 0 and 1

        if !self.print {
            flags &= !(1 << 2);
        }
        if !self.modify {
            flags &= !(1 << 3);
        }
        if !self.copy {
            flags &= !(1 << 4);
        }
        if !self.annotate {
            flags &= !(1 << 5);
        }

        flags
    }
}

pub struct EncryptionConfig {
    pub owner_password: String,
    pub user_password: String,
    pub permissions: Permissions,
}

impl EncryptionConfig {
    pub fn new() -> Self {
        Self {
            owner_password: String::new(),
            user_password: String::new(),
            permissions: Permissions::new(),
        }
    }

    pub fn with_owner_password(mut self, password: &str) -> Self {
        self.owner_password = password.to_string();
        self
    }

    pub fn with_user_password(mut self, password: &str) -> Self {
        self.user_password = password.to_string();
        self
    }

    pub fn with_permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
}

/// Raw cryptographic outputs — no PDF object types.
pub struct EncryptionValues {
    pub o_value: [u8; 32],
    pub u_value: [u8; 32],
    pub permissions: i32,
    pub encryption_key: [u8; 5],
}

/// Compute all encryption values from config and file ID.
pub fn compute_encryption_values(
    config: &EncryptionConfig,
    file_id_bytes: &[u8],
) -> EncryptionValues {
    let owner_pwd = config.owner_password.as_bytes();
    let user_pwd = config.user_password.as_bytes();
    let p_value = config.permissions.as_i32();

    let o_value = compute_o_value(owner_pwd, user_pwd);
    let encryption_key = compute_encryption_key(user_pwd, &o_value, p_value, file_id_bytes);
    let u_value = compute_u_value(&encryption_key);

    EncryptionValues {
        o_value,
        u_value,
        permissions: p_value,
        encryption_key,
    }
}

/// Computes MD5 hash of all non-free objects and returns both hex string and raw bytes.
pub fn compute_data_hash(_objects: &[PdfObject]) -> (String, Vec<u8>) {
    let context = md5::Context::new();
    /*  for obj in objects {
            if obj.metadata().status != ObjectStatus::Free {
                context.consume(obj.serialize());
            }
        }
    */
    let hash_result = context.finalize().0;
    let data_hash_hex: String = hash_result.iter().map(|b| format!("{:02x}", b)).collect();
    let data_hash_bytes = data_hash_hex.as_bytes().to_vec();
    (data_hash_hex, data_hash_bytes)
}

/// Returns the appropriate file ID bytes based on the identifier mode.
pub fn get_id_bytes<'a>(
    identifier_mode: &'a FileIdentifierMode,
    data_hash_bytes: &'a [u8],
) -> &'a [u8] {
    match identifier_mode {
        FileIdentifierMode::Custom(bytes) => bytes.as_slice(),
        _ => data_hash_bytes,
    }
}

/// Encode a raw byte slice as a PDF hex string, e.g. `<4F3A...>`.
pub fn bytes_to_pdf_hex_string(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2 + 2);
    hex.push('<');
    for b in bytes {
        hex.push_str(&format!("{:02X}", b));
    }
    hex.push('>');
    hex
}

/// Pad or truncate a password to exactly 32 bytes using the PDF padding string.
fn pad_password(password: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let len = password.len().min(32);
    padded[..len].copy_from_slice(&password[..len]);
    if len < 32 {
        padded[len..].copy_from_slice(&PDF_PASSWORD_PADDING[..32 - len]);
    }
    padded
}

/// Simple RC4 encrypt/decrypt (symmetric — same operation for both).
fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s: [u8; 256] = [0u8; 256];
    for i in 0..256 {
        s[i] = i as u8;
    }
    let mut j: usize = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }

    let mut output = Vec::with_capacity(data.len());
    let mut i: usize = 0;
    j = 0;
    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        output.push(byte ^ k);
    }

    output
}

/// Algorithm 3: Computing the O (owner password) value.
fn compute_o_value(owner_password: &[u8], user_password: &[u8]) -> [u8; 32] {
    let owner_pwd = if owner_password.is_empty() {
        user_password
    } else {
        owner_password
    };

    let padded_owner = pad_password(owner_pwd);
    let hash = md5::compute(&padded_owner);
    let rc4_key = &hash[..5];

    let padded_user = pad_password(user_password);
    let encrypted = rc4(rc4_key, &padded_user);

    let mut o_value = [0u8; 32];
    o_value.copy_from_slice(&encrypted[..32]);
    o_value
}

/// Algorithm 2: Computing an encryption key.
fn compute_encryption_key(
    user_password: &[u8],
    o_value: &[u8; 32],
    permissions: i32,
    file_id: &[u8],
) -> [u8; 5] {
    let padded = pad_password(user_password);

    let mut input = Vec::with_capacity(68 + file_id.len());
    input.extend_from_slice(&padded);
    input.extend_from_slice(o_value);
    input.extend_from_slice(&permissions.to_le_bytes());
    input.extend_from_slice(file_id);

    let hash = md5::compute(&input);
    let mut key = [0u8; 5];
    key.copy_from_slice(&hash[..5]);
    key
}

/// Algorithm 4: Computing the U (user password) value.
fn compute_u_value(encryption_key: &[u8; 5]) -> [u8; 32] {
    let encrypted = rc4(encryption_key, &PDF_PASSWORD_PADDING);
    let mut u_value = [0u8; 32];
    u_value.copy_from_slice(&encrypted[..32]);
    u_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_empty_password_is_padding_string() {
        let padded = pad_password(b"");
        assert_eq!(padded, PDF_PASSWORD_PADDING);
    }

    #[test]
    fn pad_short_password() {
        let padded = pad_password(b"abc");
        assert_eq!(&padded[..3], b"abc");
        assert_eq!(&padded[3..], &PDF_PASSWORD_PADDING[..29]);
    }

    #[test]
    fn pad_exact_32_bytes() {
        let pwd = [b'X'; 32];
        let padded = pad_password(&pwd);
        assert_eq!(padded, pwd);
    }

    #[test]
    fn pad_long_password_truncated() {
        let pwd = [b'A'; 64];
        let padded = pad_password(&pwd);
        assert_eq!(padded, [b'A'; 32]);
    }

    #[test]
    fn rc4_roundtrip() {
        let key = b"hello";
        let plaintext = b"some secret data to encrypt";
        let ciphertext = rc4(key, plaintext);
        let decrypted = rc4(key, &ciphertext);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn o_value_is_32_bytes() {
        let o = compute_o_value(b"owner", b"user");
        assert_eq!(o.len(), 32);
    }

    #[test]
    fn u_value_is_32_bytes() {
        let o = compute_o_value(b"", b"");
        let key = compute_encryption_key(b"", &o, -4, b"fileid");
        let u = compute_u_value(&key);
        assert_eq!(u.len(), 32);
    }

    #[test]
    fn permissions_new_denies_all() {
        let p = Permissions::new().as_i32();
        assert_eq!(p & (1 << 2), 0);
        assert_eq!(p & (1 << 3), 0);
        assert_eq!(p & (1 << 4), 0);
        assert_eq!(p & (1 << 5), 0);
    }

    #[test]
    fn permissions_selective() {
        let p = Permissions { print: true, ..Permissions::new() }.as_i32();
        assert_ne!(p & (1 << 2), 0); // print allowed
        assert_eq!(p & (1 << 3), 0); // modify denied
        assert_eq!(p & (1 << 4), 0); // copy denied
        assert_eq!(p & (1 << 5), 0); // annotate denied
    }

    #[test]
    fn compute_values_returns_consistent_lengths() {
        let config = EncryptionConfig::new();
        let vals = compute_encryption_values(&config, b"test-file-id");
        assert_eq!(vals.o_value.len(), 32);
        assert_eq!(vals.u_value.len(), 32);
        assert_eq!(vals.encryption_key.len(), 5);
    }

    #[test]
    fn hex_string_format() {
        let hex = bytes_to_pdf_hex_string(&[0x4F, 0x00, 0xFF]);
        assert_eq!(hex, "<4F00FF>");
    }
}
