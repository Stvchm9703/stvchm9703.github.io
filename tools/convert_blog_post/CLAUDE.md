# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool that converts blog posts exported from Anytype (a decentralized note-taking app) into a standardized JSON format for use with a web-based blogging system.

## Build & Run Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run with arguments
cargo run -- -i <import_path> -o <export_path>

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Check for issues without building
cargo check

# Auto-fix warnings (unused imports, etc.)
cargo fix --allow-dirty --allow-staged
```

### CLI Arguments

- `-i, --import-path` - Path to Anytype export directory containing `.pb` files
- `-o, --export-path` - Path for JSON output
- `-s, --skip-copy` - Skip copying asset files
- `-c, --collections` - Filter by collection names
- `-t, --ts-export` - Export TypeScript type definitions

## Architecture

### Core Data Flow

1. **Input**: Anytype protobuf (`.pb`) snapshot files from an export directory
2. **Parse**: `quick-protobuf` deserializes `.pb` files into `SnapshotWithType` structs
3. **Transform**: Objects are converted via `FromRaw` trait implementations into export models
4. **Resolve**: Cross-references between pages, files, tags, and bookmarks are resolved
5. **Output**: Serialized as JSON files organized by collection

### Key Modules

- `src/proto/anytype/` - Auto-generated protobuf definitions (DO NOT EDIT - regenerate with `pb-rs`)
- `src/export_model/` - Core domain models for export:
  - `page/` - Page/article model with content blocks, layout ordering, and external links
  - `content_block/` - Content block types (text, file, bookmark, jupyter, latex, link, etc.)
  - `tag/` - Tag/series models with options
  - `collection.rs` - Collection grouping
  - `file_object.rs` - File/image handling
  - `trait_impl.rs` - Conversion traits (`FromRaw`, `FromBlock`, `FromBlockContent`)
- `src/jupyter_notebook/` - Jupyter notebook cell parsing and rendering

### Key Traits

- `FromRaw<T>` - Convert protobuf types to export models
- `FromBlock` - Convert Anytype blocks to `ContentBlock`
- `FromBlockContent<T>` - Convert block content to component attributes

### Global State

- `GLOBAL_RELATION_NAMEMAP` / `GLOBAL_RELATION_IDMAP` - Tag/relation lookup tables
- `DEFAULT_TAG` - Default tag set for page categorization

## Regenerating Protobuf Definitions

Proto source files are in `protos/anytype/` (git submodule from https://github.com/anyproto/any-block).

```bash
pb-rs -I=protos/anytype -o=src/proto/anytype/snapshot.rs protos/anytype/snapshot.proto
pb-rs -I=protos/anytype -o=src/proto/anytype/models.rs protos/anytype/models.proto -s --owned
pb-rs -I=protos/anytype -o=src/proto/anytype/events.rs protos/anytype/events.proto -s
pb-rs -I=protos/anytype -o=src/proto/anytype/changes.rs protos/anytype/changes.proto -s
```

## Jupyter Notebook Sidecar Pipeline Order

When a blog page contains Jupyter notebook blocks, the sidecar pre-processor
**must run before** `convert_blog_post` so that data tables are available at
conversion time.

Required order for each `.ipynb` that lives in the Anytype export `files/` dir:

```bash
# Step 1 — generate the sidecar next to the .ipynb (once per notebook)
ipynb_data_visualization <import>/files/<notebook>.ipynb --sidecar

# Step 2 — convert the Anytype export (reads the sidecar automatically)
convert_blog_post -i <import> -o <export>
```

`--sidecar` writes `<import>/files/<notebook>.viz.json` next to the `.ipynb`.
`convert_blog_post` copies all `files/*` to the export directory and, when it
reads each `.ipynb`, it records the **absolute filesystem path** in
`JupyterNotebookRoot::source_path`.  During page resolution the notebook
resolver calls `JupyterComponentAttr::load_sidecar(source_path)` which reads
`<parent>/<stem>.viz.json` and populates `data_tables` for the matching
`cellIndex`.  If no sidecar is present the field stays `null` and the blog
renders without data tables (zero impact on existing pages).

## Important Notes

- Files in `src/proto/anytype/` are auto-generated - do not modify directly
- The macro crate `convert_blog_post_marco/` provides the `set_field_value!` macro for field extraction
- Uses Rust Edition 2024
