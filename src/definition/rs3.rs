#[allow(clippy::too_many_lines)]
mod item_def;

pub use item_def::*;

use crate::Cache;
use runefs::{IndexMetadata, ArchiveFileGroup, REFERENCE_TABLE_ID};
use std::collections::HashMap;

pub(crate) const ID_BLOCK_SIZE: usize = 256;

/// Marker trait for definitions.
pub trait Definition: Sized {
    fn new(id: u32, buffer: &[u8]) -> crate::Result<Self>;
}

/// Adds definition fetching from the cache to every struct that implements `Definition`.
pub trait FetchDefinition: Definition {
    // TODO: example
    /// Fetches multiple definitions from every archive in the index.
    ///
    /// Note: every archive contains only one definition. (1:1)
    ///
    /// # Errors
    ///
    /// Can return multiple errors: if reading, decoding or parsing definition buffers fail.
    fn fetch_from_index<D>(cache: &Cache, index_id: u8) -> crate::Result<HashMap<u32, D>>
    where
        D: Definition,
    {
        let buffer = cache.read(REFERENCE_TABLE_ID, index_id as u32)?.decode()?;
        let archives = IndexMetadata::from_buffer(buffer)?;

        let mut definitions = std::collections::HashMap::new();
        let mut base_id = 0;

        for archive in &archives {
            let buffer = cache.read(index_id, archive.id as u32)?.decode()?;

            let archive_group = ArchiveFileGroup::from_buffer(&buffer, archive.entry_count);

            for archive_file in archive_group {
                let id = base_id + archive.valid_ids[archive_file.id as usize] as usize;
                definitions.insert(id as u32, D::new(id as u32, &archive_file.data)?);
            }

            base_id += ID_BLOCK_SIZE;
        }

        Ok(definitions)
    }
}

impl<D: Definition> FetchDefinition for D {}
