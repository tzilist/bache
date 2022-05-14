use futures::future;
use tokio::sync::OnceCell;

pub struct DigestInfo {
    pub size_bytes: i64,
    pub packed_hash: [u8; 32],
    str_hash: OnceCell<String>,
}

impl DigestInfo {
    pub fn new(packed_hash: [u8; 32], size_bytes: i64) -> Self {
        Self {
            size_bytes,
            packed_hash,
            str_hash: OnceCell::const_new(),
        }
    }

    pub async fn hash_as_str(&self) -> &str {
        self.str_hash
            .get_or_init(|| async { future::ready("Hello".into()).await })
            .await
            .as_str()
    }
}
