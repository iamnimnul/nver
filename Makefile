APP_NAME := nver
DIST_DIR := dist

CARGO ?= $(shell command -v cargo 2>/dev/null)
RUSTUP ?= $(shell command -v rustup 2>/dev/null)

ifeq ($(CARGO),)
CARGO := $(HOME)/.cargo/bin/cargo
endif

ifeq ($(RUSTUP),)
RUSTUP := $(HOME)/.cargo/bin/rustup
endif

TARGET_MACOS_ARM64 := aarch64-apple-darwin
TARGET_MACOS_X86_64 := x86_64-apple-darwin
TARGET_LINUX_X86_64 := x86_64-unknown-linux-gnu
TARGET_LINUX_ARM64 := aarch64-unknown-linux-gnu
TARGET_WINDOWS_X86_64 := x86_64-pc-windows-gnu

.PHONY: help build test release clean \
	build-macos-arm64 build-macos-x86_64 \
	build-linux-x86_64 build-linux-arm64 \
	build-windows-x86_64

help:
	@echo "Common targets:"
	@echo "  make build                 Build debug binary"
	@echo "  make test                  Run tests"
	@echo "  make release               Build release binary"
	@echo "  make clean                 Remove build artifacts"
	@echo ""
	@echo "Cross-platform builds:"
	@echo "  make build-macos-arm64"
	@echo "  make build-macos-x86_64"
	@echo "  make build-linux-x86_64"
	@echo "  make build-linux-arm64"
	@echo "  make build-windows-x86_64"

build:
	$(CARGO) build

test:
	$(CARGO) test

release:
	$(CARGO) build --release

clean:
	$(CARGO) clean
	rm -rf $(DIST_DIR)

build-macos-arm64:
	@if [ ! -x "$(RUSTUP)" ]; then \
		echo "rustup was not found. Install rustup from https://rustup.rs and try again."; \
		exit 1; \
	fi
	$(RUSTUP) target add $(TARGET_MACOS_ARM64)
	$(CARGO) build --release --target $(TARGET_MACOS_ARM64)
	mkdir -p $(DIST_DIR)/$(TARGET_MACOS_ARM64)
	cp target/$(TARGET_MACOS_ARM64)/release/$(APP_NAME) $(DIST_DIR)/$(TARGET_MACOS_ARM64)/

build-macos-x86_64:
	@if [ ! -x "$(RUSTUP)" ]; then \
		echo "rustup was not found. Install rustup from https://rustup.rs and try again."; \
		exit 1; \
	fi
	$(RUSTUP) target add $(TARGET_MACOS_X86_64)
	$(CARGO) build --release --target $(TARGET_MACOS_X86_64)
	mkdir -p $(DIST_DIR)/$(TARGET_MACOS_X86_64)
	cp target/$(TARGET_MACOS_X86_64)/release/$(APP_NAME) $(DIST_DIR)/$(TARGET_MACOS_X86_64)/

build-linux-x86_64:
	@if [ ! -x "$(RUSTUP)" ]; then \
		echo "rustup was not found. Install rustup from https://rustup.rs and try again."; \
		exit 1; \
	fi
	$(RUSTUP) target add $(TARGET_LINUX_X86_64)
	$(CARGO) build --release --target $(TARGET_LINUX_X86_64)
	mkdir -p $(DIST_DIR)/$(TARGET_LINUX_X86_64)
	cp target/$(TARGET_LINUX_X86_64)/release/$(APP_NAME) $(DIST_DIR)/$(TARGET_LINUX_X86_64)/

build-linux-arm64:
	@if [ ! -x "$(RUSTUP)" ]; then \
		echo "rustup was not found. Install rustup from https://rustup.rs and try again."; \
		exit 1; \
	fi
	$(RUSTUP) target add $(TARGET_LINUX_ARM64)
	$(CARGO) build --release --target $(TARGET_LINUX_ARM64)
	mkdir -p $(DIST_DIR)/$(TARGET_LINUX_ARM64)
	cp target/$(TARGET_LINUX_ARM64)/release/$(APP_NAME) $(DIST_DIR)/$(TARGET_LINUX_ARM64)/

build-windows-x86_64:
	@if [ ! -x "$(RUSTUP)" ]; then \
		echo "rustup was not found. Install rustup from https://rustup.rs and try again."; \
		exit 1; \
	fi
	$(RUSTUP) target add $(TARGET_WINDOWS_X86_64)
	$(CARGO) build --release --target $(TARGET_WINDOWS_X86_64)
	mkdir -p $(DIST_DIR)/$(TARGET_WINDOWS_X86_64)
	cp target/$(TARGET_WINDOWS_X86_64)/release/$(APP_NAME).exe $(DIST_DIR)/$(TARGET_WINDOWS_X86_64)/
