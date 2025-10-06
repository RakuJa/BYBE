use async_compression::tokio::write::ZstdEncoder;
use base64ct::{Base64Url, Encoding};
use postcard::to_allocvec;
use serde::Serialize;
use tokio::io::{
    AsyncWriteExt as _, // for `write_all` and `shutdown`
};

pub trait Base64Encode: Serialize {
    async fn encode(&self) -> anyhow::Result<String> {
        let raw_binary = to_allocvec(&self)?;
        let compressed_binary = Self::compress_binary_data(raw_binary).await?;
        Ok(Base64Url::encode_string(compressed_binary.as_slice()))
    }

    async fn compress_binary_data(input: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let mut encoder = ZstdEncoder::new(Vec::new());
        encoder.write_all(input.as_slice()).await?;
        encoder.shutdown().await?;
        Ok(encoder.into_inner())
    }
}
