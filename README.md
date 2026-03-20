# nver

nver is a cross-platform Rust CLI that finds the latest Git version tag,
bumps semantic version by major, minor, or patch, and creates an annotated
tag with commit messages since the previous release.

## Supported Tag Formats

- X.X.X
- vX.X.X
- v.X.X.X

## Prerequisites

- Rust toolchain (cargo, rustc)
- Git
- Make

## Install Rust

Use rustup:

curl https://sh.rustup.rs -sSf | sh

Then restart your shell and verify:

cargo --version
rustc --version

## Build and Test

- Debug build:
  make build
- Run tests:
  make test
- Optimized build:
  make release

## Cross-Platform Builds

The Makefile includes targets for common platforms.

- macOS (Apple Silicon):
  make build-macos-arm64
- macOS (Intel):
  make build-macos-x86_64
- Linux x86_64:
  make build-linux-x86_64
- Linux aarch64:
  make build-linux-arm64
- Windows x86_64:
  make build-windows-x86_64

Build artifacts are copied into dist/<target-triple>/.

## CLI Usage

Dry run (does not create a tag):

cargo run -- patch --dry-run

Create a new tag:

cargo run -- minor

Binary usage after build:

./target/debug/nver patch --dry-run

## What nver Does

1. Finds the latest valid version tag from Git tags.
2. Parses one of the accepted formats.
3. Bumps major, minor, or patch.
4. Collects commit subjects since previous tag.
5. Creates an annotated tag whose message contains those commits.

## Notes

- If no valid version tag is found, nver exits with a clear error message.
- Created tags are local. Push with:

git push origin --tags
