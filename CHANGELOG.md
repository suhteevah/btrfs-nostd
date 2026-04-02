# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Superblock parsing and serialization with CRC32C validation
- B-tree traversal (search, walk, collect) with binary search
- B-tree modification (leaf insert, leaf split)
- CRC32C (Castagnoli) software implementation with lookup table
- Chunk map with logical-to-physical address translation
- sys_chunk_array bootstrap parsing from superblock
- Full chunk tree reading for complete address mapping
- Inode item read/write with timestamps, permissions, file types
- Directory entry support (DIR_ITEM hash lookup, DIR_INDEX ordered iteration)
- File extent support (inline, regular, prealloc, hole detection)
- High-level API: `BtrFs::mount`, `read_file`, `write_file`, `mkdir`, `list_dir`
- `BlockDevice` trait for pluggable storage backends
- Comprehensive `BtrfsError` error type
- Path resolution with component-by-component directory traversal
- `no_std` compatible with only `alloc` and `log` dependencies
