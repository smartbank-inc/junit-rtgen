# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

junit-rtgen is a Rust CLI tool that converts JUnit XML format files to ParallelTests::RSpec::RuntimeLogger format. This conversion helps optimize parallel test execution in Ruby/RSpec projects.

## Commands

### Build & Development
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release binary
- `cargo run` - Run the application in debug mode
- `cargo run --release` - Run the optimized release version

### Testing
- `cargo test` - Run all tests (8 unit tests + 1 integration test)
- `cargo test [test_name]` - Run a specific test (e.g. `cargo test test_split_xml`)
- `cargo test --lib` - Run unit tests only (tests in src/lib.rs)
- `cargo test --test integration_test` - Run integration test only

### Code Quality
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo fmt --all -- --check` - Check if code is properly formatted (CI command)
- `cargo clippy` - Run the Rust linter for code improvements
- `cargo clippy --all-targets --all-features -- -D warnings` - Run clippy with all targets and treat warnings as errors (CI command)

### Documentation
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open documentation in browser

## Architecture

The project is structured as a Rust library with CLI wrapper, implementing dual XML parsing approaches:

### Core Components

1. **Library (`src/lib.rs`)** - Contains all core functionality:
   - `TestSuite`, `TestCase` structs with serde deserialization
   - `process_junit_xml()` - Serde-based parser for full documents
   - `process_junit_xml_streaming()` - Memory-efficient streaming parser
   - `split_xml_documents()` - Handles concatenated XML files

2. **CLI Application (`src/main.rs`)** - Minimal wrapper that:
   - Reads from stdin using streaming parser
   - Outputs in `file:time` format to stdout

3. **Data Flow**:
   - Input: JUnit XML via stdin
   - Processing: Extract `file` and `time` attributes from `<testcase>` elements
   - Aggregation: Sum times per file path in HashMap
   - Output: ParallelTests::RSpec::RuntimeLogger format

### Key Implementation Details

- **Dual Parsing Strategy**: Streaming (memory-efficient) and serde-based (full-featured)
- **Multi-document Support**: Handles multiple XML files concatenated in single stream
- **Error Resilience**: Continues processing on malformed XML sections
- **Memory Optimization**: Uses `std::mem::take()`, pre-allocated capacity, direct f64 deserialization
- **Dependencies**: `quick-xml` for parsing, `serde` for deserialization

### Test Architecture

- **Unit tests**: In `lib.rs` covering edge cases and both parsing approaches
- **Integration tests**: In `tests/` directory for end-to-end scenarios
- **8 test cases** covering splitting logic, streaming parser, floating-point precision

## Development Guidelines

- This project uses Rust edition 2024
- Main dependencies: `quick-xml` (XML parsing), `serde` (deserialization)
- The CLI uses streaming parser (`process_junit_xml_streaming`) for memory efficiency
- Both parsing approaches are available: streaming and serde-based
- Tests include floating-point comparisons with tolerance for time values
- Error handling continues processing on malformed XML sections

## CI/CD

### Continuous Integration (`.github/workflows/rust.yml`)
Runs on push to main and pull requests:
- Runs `cargo check` to verify compilation
- Runs `cargo fmt --all -- --check` to ensure consistent formatting
- Runs `cargo clippy --all-targets --all-features -- -D warnings` to catch code issues
- Runs `cargo test` to execute all tests

### Release Process (`.github/workflows/release.yml`)
Manual release workflow triggered via GitHub Actions `workflow_dispatch`:
- Input: Release version (e.g., `v1.0.0`)
- Generates release notes from commit history since last tag
- Builds binaries for multiple platforms:
  - Linux (x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl)
  - macOS (x86_64-apple-darwin, aarch64-apple-darwin)
  - Windows (x86_64-pc-windows-msvc)
- Creates GitHub release with downloadable binary archives
- No crates.io publishing (binaries-only distribution)