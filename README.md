# junit-rtgen

A CLI tool that converts JUnit XML format files to ParallelTests::RSpec::RuntimeLogger format.

## Features

- Parses JUnit XML format files from stdin
- Extracts file paths and execution times from test cases
- Groups test results by file and sums the execution times
- Outputs results in ParallelTests::RSpec::RuntimeLogger format (`file:total_time`)
- Supports processing multiple XML files in a single stream
- Memory-efficient streaming parser for handling large files

## Installation

### Download Pre-built Binaries (Recommended)

Download the latest release for your platform from the [GitHub Releases](https://github.com/smartbank-inc/junit-rtgen/releases) page:

- **Linux (x86_64)**: `junit-rtgen-vX.X.X-x86_64-unknown-linux-gnu.tar.gz`
- **Linux (musl)**: `junit-rtgen-vX.X.X-x86_64-unknown-linux-musl.tar.gz`
- **macOS (Intel)**: `junit-rtgen-vX.X.X-x86_64-apple-darwin.tar.gz`
- **macOS (Apple Silicon)**: `junit-rtgen-vX.X.X-aarch64-apple-darwin.tar.gz`
- **Windows**: `junit-rtgen-vX.X.X-x86_64-pc-windows-msvc.zip`

```bash
# Example for Linux
curl -LO https://github.com/smartbank-inc/junit-rtgen/releases/latest/download/junit-rtgen-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
tar xzf junit-rtgen-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
chmod +x junit-rtgen
sudo mv junit-rtgen /usr/local/bin/
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/smartbank-inc/junit-rtgen.git
cd junit-rtgen

# Build the release version
cargo build --release

# The binary will be available at target/release/junit-rtgen
```

## Usage

```bash
# Single file
cat junit-report.xml | junit-rtgen > runtime.log

# Multiple files
cat *.xml | junit-rtgen > runtime.log

# Direct input
junit-rtgen < junit-report.xml

# From RSpec with JUnit formatter (https://github.com/sj26/rspec_junit_formatter)
rspec --format RspecJunitFormatter | junit-rtgen > runtime.log
```

## Input Format

The tool expects JUnit XML format with test cases containing `file` and `time` attributes:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="3">
  <testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
  <testcase classname="spec.bar_spec" name="Bar spec" file="./spec/bar.rb" time="0.22222"></testcase>
</testsuite>
```

## Output Format

The output follows the ParallelTests::RSpec::RuntimeLogger format:

```
./spec/foo.rb:0.11111
./spec/bar.rb:0.22222
```

When multiple test cases reference the same file, their times are summed:

```
./spec/foo.rb:0.45678
./spec/bar.rb:0.10101
```

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Build and run
cargo run < sample.xml
```

## License

MIT License (see LICENSE file)

## References

Inspiration from the following article:
- https://qiita.com/tomoasleep/items/0ee5cae02739c8695c59
