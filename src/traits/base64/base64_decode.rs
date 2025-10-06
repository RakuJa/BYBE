use async_compression::tokio::bufread::ZstdDecoder;
use base64ct::{Base64Url, Encoding};
use postcard::from_bytes;
use serde::de::DeserializeOwned;
use std::io::Cursor;
use tokio::io::AsyncReadExt;

pub trait Base64Decode: DeserializeOwned {
    async fn decode(base64_data: String) -> anyhow::Result<Self> {
        let compressed_binary = Base64Url::decode_vec(base64_data.as_str())?;
        let decompressed_binary = Self::decompress_binary(compressed_binary).await?;
        Ok(from_bytes(&decompressed_binary)?)
    }

    async fn decompress_binary(compressed: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(compressed.as_slice());
        let mut decoder = ZstdDecoder::new(cursor);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).await?;
        Ok(decompressed)
    }
}
