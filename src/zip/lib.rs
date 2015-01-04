#![feature(macro_rules, default_type_params, associated_types)]
#![feature(old_orphan_check)] // XXX maybe_utf8 requires this

extern crate flate;

pub use self::fileinfo::{CompressionMethod, FileInfo};
pub use self::reader::ZipReader;

mod crc32;
pub mod maybe_utf8;
pub mod error;
pub mod format;
pub mod fileinfo;
pub mod reader;

