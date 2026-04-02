//! # btrfs-nostd
//!
//! A `no_std` btrfs filesystem implementation in pure Rust with read and write support.
//!
//! This crate provides read and write access to btrfs filesystems without requiring
//! the standard library, making it suitable for bare-metal, embedded, and OS kernel
//! environments.
//!
//! ## Features
//!
//! - Superblock parsing and validation (at offset 0x10000)
//! - B-tree traversal and modification (search, insert, split)
//! - Inode item reading and writing
//! - Directory entry parsing, creation, and lookup (CRC32C name hashing)
//! - File extent data (inline, regular, prealloc)
//! - Chunk/device mapping (logical to physical address translation)
//! - CRC32C checksums (superblock, metadata, directory name hashing)
//! - High-level file read/write/create/mkdir/list_dir API
//!
//! ## Usage
//!
//! ```rust,no_run
//! use btrfs_nostd::{BtrFs, BlockDevice, BtrfsError};
//!
//! struct MyDevice { /* your storage backend */ }
//!
//! impl BlockDevice for MyDevice {
//!     fn read_bytes(&self, offset: u64, buf: &mut [u8]) -> Result<(), BtrfsError> {
//!         // Read from your block device at the given byte offset
//!         todo!()
//!     }
//!
//!     fn write_bytes(&self, offset: u64, buf: &[u8]) -> Result<(), BtrfsError> {
//!         // Write to your block device at the given byte offset
//!         todo!()
//!     }
//! }
//!
//! // Mount the filesystem
//! let device = MyDevice { /* ... */ };
//! let fs = BtrFs::mount(device).expect("failed to mount btrfs");
//!
//! // Read a file
//! let data = fs.read_file(b"/etc/hostname").expect("read failed");
//!
//! // List a directory
//! let entries = fs.list_dir(b"/").expect("list failed");
//! ```

#![no_std]

extern crate alloc;

pub mod chunk;
pub mod crc32c;
pub mod dir;
pub mod extent;
pub mod inode;
pub mod item;
pub mod key;
pub mod readwrite;
pub mod superblock;
pub mod tree;

pub use readwrite::{BlockDevice, BtrFs, BtrfsError};
pub use superblock::Superblock;
pub use key::{BtrfsKey, KeyType};
pub use item::{BtrfsHeader, BtrfsLeaf, BtrfsNode, BtrfsKeyPtr};
pub use inode::InodeItem;
pub use dir::DirItem;
pub use extent::FileExtentItem;
pub use chunk::{ChunkItem, Stripe};
