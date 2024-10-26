use hmac::{Hmac, Mac};
use sha2::Sha256;

// Type alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// Computes HMAC-SHA256 for the given key and data, returning a hexadecimal `String`.
///
/// # Arguments
///
/// * `key` - The key used for HMAC, accepts `&str`, `String`, or `&[u8]`.
/// * `data` - The data to be hashed, accepts `&str`, `String`, or `&[u8]`.
///
/// # Returns
///
/// A `String` containing the HMAC-SHA256 result in hexadecimal format.
pub fn hmac_sha256<K, D>(key: K, data: D) -> String
where
    K: AsRef<[u8]>,
    D: AsRef<[u8]>,
{
    let mut mac = HmacSha256::new_from_slice(key.as_ref()).expect("HMAC can take key of any size");
    mac.update(data.as_ref());
    let result = mac.finalize().into_bytes();

    // Convert to hexadecimal string
    hex::encode(result)
}