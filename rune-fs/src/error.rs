//! Error management.

use std::{error::Error, fmt, io};

/// A specialized result type for cache operations.
///
/// This type is broadly used across rscache for any operation which may produce a
/// [RuneFsError](enum.RuneFsError.html).
///
/// # Examples
///
/// A convenience function that bubbles an `rscache::Result` to its caller:
///
/// ```
/// use rscache::Cache;
/// use rscache::codec;
///
/// // Same result as Result<Vec<u8>, RuneFsError>
/// fn item_def_data(cache: &Cache) -> rscache::Result<Vec<u8>> {
///     let index_id = 2;
///     let archive_id = 10;
///
///     let buffer = cache.read(index_id, archive_id)?;
///     let buffer = codec::decode(&buffer)?;
///
///     Ok(buffer)
/// }
/// ```
pub type Result<T> = std::result::Result<T, RuneFsError>;

/// Super error type for all sub cache errors
#[derive(Debug)]
pub enum RuneFsError {
    /// Wrapper for the std::io::Error type.
    Io(io::Error),
    Read(ReadError),
    Compression(CompressionUnsupported),
    /// Clarification error for failed parsers.
    Parse(ParseError),
    Validate(ValidateError),
}

macro_rules! impl_from {
    ($ty:path, $var:ident) => {
        impl From<$ty> for RuneFsError {
            #[inline]
            fn from(err: $ty) -> Self {
                Self::$var(err)
            }
        }
    };
}

impl_from!(io::Error, Io);
impl_from!(ReadError, Read);
impl_from!(CompressionUnsupported, Compression);
impl_from!(ParseError, Parse);
impl_from!(ValidateError, Validate);

impl From<nom::Err<()>> for RuneFsError {
    #[inline]
    fn from(_: nom::Err<()>) -> Self {
        Self::Parse(ParseError::Unknown)
    }
}

impl Error for RuneFsError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Read(err) => Some(err),
            Self::Compression(err) => Some(err),
            Self::Parse(err) => Some(err),
            Self::Validate(err) => Some(err),
        }
    }
}

impl fmt::Display for RuneFsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Read(err) => err.fmt(f),
            Self::Compression(err) => err.fmt(f),
            Self::Parse(err) => err.fmt(f),
            Self::Validate(err) => err.fmt(f),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ReadError {
    IndexNotFound(u8),
    ArchiveNotFound(u8, u32),
    ReferenceTableNotFound,
    NameNotInArchive(i32, String, u8),
    SectorArchiveMismatch(u32, u32),
    SectorChunkMismatch(usize, usize),
    SectorNextMismatch(u32, u32),
    SectorIndexMismatch(u8, u8),
}

impl Error for ReadError {}

impl fmt::Display for ReadError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndexNotFound(id) => write!(f, "Index {} not found.", id),
            Self::ArchiveNotFound(index_id, archive_id) => write!(
                f,
                "Index {} does not contain archive group {}.",
                index_id, archive_id
            ),
            Self::ReferenceTableNotFound => write!(f, "Reference table (index 255) not found."),
            Self::NameNotInArchive(hash, name, index_id) => write!(
                f,
                "Identifier hash {} for name {} not found in index {}.",
                hash, name, index_id
            ),
            Self::SectorArchiveMismatch(received, expected) => write!(
                f,
                "Sector archive id was {} but expected {}.",
                received, expected
            ),
            Self::SectorChunkMismatch(received, expected) => write!(
                f,
                "Sector chunk was {} but expected {}.",
                received, expected
            ),
            Self::SectorNextMismatch(received, expected) => {
                write!(f, "Sector next was {} but expected {}.", received, expected)
            }
            Self::SectorIndexMismatch(received, expected) => write!(
                f,
                "Sector parent index id was {} but expected {}.",
                received, expected
            ),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompressionUnsupported;
impl Error for CompressionUnsupported {}

impl fmt::Display for CompressionUnsupported {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported compression type")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParseError {
    Unknown,
    Archive(u32),
    Sector(usize),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown parser error."),
            Self::Archive(id) => write!(f, "Unable to parse archive {}, unexpected eof.", id),
            Self::Sector(id) => write!(
                f,
                "Unable to parse child sector of parent {}, unexpected eof.",
                id
            ),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValidateError {
    InvalidLength(usize, usize),
    InvalidCrc(u32, u32, usize),
}

impl Error for ValidateError {}

impl fmt::Display for ValidateError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength(expected, actual) => {
                write!(f, "Expected length of {} but was {}.", expected, actual)
            }
            Self::InvalidCrc(expected, actual, index) => write!(
                f,
                "Mismatch crc at index {}, expected {} but was {}.",
                index, expected, actual
            ),
        }
    }
}