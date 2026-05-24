//! Asset Compression Pipeline using lz4_flex and zstd

use crate::core::error::EngineResult;

/// Compression algorithm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    LZ4,
    ZSTD,
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::None
    }
}

impl CompressionAlgorithm {
    /// Compress data using the selected algorithm
    pub fn compress(&self, data: &[u8]) -> EngineResult<Vec<u8>> {
        match self {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::LZ4 => {
                let compressed = lz4_flex::compress(data);
                Ok(compressed)
            }
            CompressionAlgorithm::ZSTD => {
                let compressed = zstd::stream::encode_all(data, 3)?;
                Ok(compressed)
            }
        }
    }
    
    /// Decompress data using the selected algorithm
    pub fn decompress(&self, data: &[u8], original_size: usize) -> EngineResult<Vec<u8>> {
        match self {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::LZ4 => {
                let decompressed = lz4_flex::decompress(data, original_size)?;
                Ok(decompressed)
            }
            CompressionAlgorithm::ZSTD => {
                let decompressed = zstd::stream::decode_all(data)?;
                Ok(decompressed)
            }
        }
    }
}

/// Compressed asset structure with verification
#[derive(Debug, Clone)]
pub struct CompressedAsset {
    pub algorithm: CompressionAlgorithm,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub hash: u64,
}

impl CompressedAsset {
    /// Create a new compressed asset
    pub fn new(algorithm: CompressionAlgorithm, data: &[u8]) -> EngineResult<Self> {
        let original_size = data.len();
        let hash = fxhash::hash64(data);
        let compressed_data = algorithm.compress(data)?;
        
        Ok(Self {
            algorithm,
            compressed_data,
            original_size,
            hash,
        })
    }
    
    /// Decompress and verify the asset
    pub fn decompress_and_verify(&self) -> EngineResult<Vec<u8>> {
        let decompressed = self.algorithm.decompress(&self.compressed_data, self.original_size)?;
        
        // Verify hash matches
        let computed_hash = fxhash::hash64(&decompressed);
        if computed_hash != self.hash {
            return Err(crate::core::error::EngineError::AssetLoading(
                "Hash mismatch after decompression".to_string()
            ));
        }
        
        Ok(decompressed)
    }
    
    /// Get compression ratio
    pub fn compression_ratio(&self) -> f32 {
        if self.original_size == 0 {
            return 0.0;
        }
        self.compressed_data.len() as f32 / self.original_size as f32
    }
}
