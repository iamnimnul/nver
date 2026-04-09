# nver

nver is a lightweight cross-platform Rust CLI that reads your latest Git version tag, bumps major, minor, or patch, and creates a new annotated release tag. It supports semantic tags in three styles (1.2.3, v1.2.3, v.1.2.3), preserves the original style when bumping, and includes commit messages since the previous tag in the annotation.

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

```sh
curl https://sh.rustup.rs -sSf | sh
```

Then restart your shell and verify:

```sh
cargo --version
rustc --version
```

## Build and Test

- Debug build:
  ```sh
  make build
  ```
- Run tests:
  ```sh
  make test
  ```
- Optimized build:
  ```sh
  make release
  ```

## Cross-Platform Builds

The Makefile includes targets for common platforms.

- macOS (Apple Silicon):
  ```sh
  make build-macos-arm64
  ```
- macOS (Intel):
  ```sh
  make build-macos-x86_64
  ```
- Linux x86_64:
  ```sh
  make build-linux-x86_64
  ```
- Linux aarch64:
  ```sh
  make build-linux-arm64
  ```
- Windows x86_64:
  ```sh
  make build-windows-x86_64
  ```

Build artifacts are copied into `dist/<target-triple>/`.

## GitHub Release Artifacts

The release workflow in `.github/workflows/release.yml` builds and uploads these executables:

- `nver-linux-x86_64`
- `nver-linux-x86_64-musl`
- `nver-macos-x86_64`
- `nver-macos-arm64`
- `nver-windows-x86_64.exe`

The workflow is triggered on tag pushes and publishes assets to the GitHub Release for that tag.

## CLI Usage

Dry run (does not create a tag):

```sh
cargo run -- patch --dry-run
```

Create a new tag:

```sh
cargo run -- minor
```

Binary usage after build:

```sh
./target/debug/nver patch --dry-run
```

## What nver Does

1. Finds the latest valid version tag from Git tags.
2. Parses one of the accepted formats.
3. Bumps major, minor, or patch.
4. Collects commit subjects since previous tag.
5. Creates an annotated tag whose message contains those commits.

## Notes

- If no valid version tag is found, nver exits with a clear error message.
- After creating a tag, nver asks whether to push it to `origin`.
- If you answer `n`, nver prints the exact `git push origin <tag>` command without executing it.
