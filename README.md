# btrfs-nostd

[![no_std](https://img.shields.io/badge/no__std-yes-green)](https://rust-embedded.github.io/book/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

A `no_std` btrfs filesystem implementation in pure Rust with read and write support.

Designed for bare-metal, embedded, and OS kernel environments where the Rust standard
library is not available. The only dependency is `log` for optional diagnostic output.

## Features

- **Superblock** parsing and validation (primary at 0x10000, mirrors supported)
- **B-tree traversal** with binary search (search, walk, collect)
- **B-tree modification** (insert into leaf, leaf splitting with COW semantics)
- **CRC32C checksums** (Castagnoli) for superblock, metadata nodes, and directory name hashing
- **Chunk mapping** -- logical to physical address translation via sys_chunk_array bootstrap and full chunk tree
- **Inode items** -- read/write metadata (size, mode, timestamps, permissions, uid/gid)
- **Directory entries** -- DIR_ITEM (hash-keyed O(1) lookup) and DIR_INDEX (ordered iteration)
- **File extents** -- inline data (small files in B-tree leaves), regular extents, prealloc, hole detection
- **High-level API** -- `read_file`, `write_file`, `mkdir`, `list_dir`, path resolution

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
btrfs-nostd = "0.1"
```

Implement the `BlockDevice` trait for your storage backend:

```rust
use btrfs_nostd::{BtrFs, BlockDevice, BtrfsError};

struct RamDisk {
    data: Vec<u8>,
}

impl BlockDevice for RamDisk {
    fn read_bytes(&self, offset: u64, buf: &mut [u8]) -> Result<(), BtrfsError> {
        let start = offset as usize;
        let end = start + buf.len();
        if end > self.data.len() {
            return Err(BtrfsError::IoError);
        }
        buf.copy_from_slice(&self.data[start..end]);
        Ok(())
    }

    fn write_bytes(&self, offset: u64, buf: &[u8]) -> Result<(), BtrfsError> {
        // For a real implementation, write to your storage
        Err(BtrfsError::IoError)
    }
}

// Mount and use:
let device = RamDisk { data: disk_image };
let fs = BtrFs::mount(device).expect("mount failed");

let contents = fs.read_file(b"/hello.txt").expect("read failed");
let entries = fs.list_dir(b"/").expect("listdir failed");
```

## Supported on-disk structures

| Structure | Read | Write |
|-----------|------|-------|
| Superblock | Yes | Yes |
| B-tree nodes (leaf + internal) | Yes | Yes |
| Chunk items / stripe mapping | Yes | Yes |
| Inode items | Yes | Yes |
| Directory items (DIR_ITEM, DIR_INDEX) | Yes | Yes |
| File extents (inline) | Yes | Yes |
| File extents (regular) | Yes | Partial |
| File extents (prealloc) | Yes | No |
| Compressed extents | No | No |

## Limitations

- Single-device only (uses first stripe for RAID configurations)
- No compression support (zlib, LZO, ZSTD extents return an error)
- Large file writes currently use inline extents (no data extent allocation)
- No extent tree updates (free space tracking)
- No transaction/journal support
- File overwrite creates a new inode rather than updating in-place
- Leaf splitting during writes is detected but not yet wired into the write path

## Architecture

The crate is organized into focused modules:

- `superblock` -- On-disk superblock layout, parsing, serialization
- `key` -- B-tree key types and ordering
- `item` -- Tree node headers, leaf items, internal node key pointers
- `tree` -- B-tree search, walk, insert, split operations
- `chunk` -- Chunk/stripe mapping, logical-to-physical address resolution
- `crc32c` -- Software CRC32C (Castagnoli polynomial) implementation
- `inode` -- Inode item metadata
- `dir` -- Directory entry parsing, creation, name hashing
- `extent` -- File extent items (inline, regular, prealloc)
- `readwrite` -- High-level `BtrFs` type tying everything together

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions welcome! Please open an issue or pull request on GitHub.

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
