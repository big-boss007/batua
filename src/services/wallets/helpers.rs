use uuid::Uuid;

const BEARER_CODE_LENGTH: usize = 16;
const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[tracing::instrument]
pub fn generate_bearer_code() -> String {
    let mut code = String::with_capacity(BEARER_CODE_LENGTH);
    let bytes_needed = (BEARER_CODE_LENGTH + 15) / 16 * 16;
    let mut raw_bytes = Vec::with_capacity(bytes_needed);

    while raw_bytes.len() < bytes_needed {
        let id = Uuid::new_v4();
        raw_bytes.extend_from_slice(id.as_bytes());
    }

    for &byte in raw_bytes.iter().take(BEARER_CODE_LENGTH) {
        let idx = (byte as usize) % ALPHANUMERIC.len();
        code.push(ALPHANUMERIC[idx] as char);
    }

    code
}
