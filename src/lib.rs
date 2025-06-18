// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];

    let bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;

    let version = u32::from_le_bytes(bytes.try_into().map_err(|_| "Failed to convert to u32")?);

    Ok(version)
}
