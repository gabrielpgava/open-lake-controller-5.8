# ==============================================================================
# Open Lake Controller v5.8 - Multi-Platform Makefile
# ==============================================================================

APP_NAME = lake-controller
VERSION = 5.8.0
DIST_DIR = dist
RELEASE_FLAGS = --release

# Targets
TARGET_WINDOWS_GNU = x86_64-pc-windows-gnu
TARGET_WINDOWS_MSVC = x86_64-pc-windows-msvc
TARGET_LINUX = x86_64-unknown-linux-gnu
TARGET_MACOS_X86 = x86_64-apple-darwin
TARGET_MACOS_ARM = aarch64-apple-darwin

.PHONY: help build release run clean screenshot check all-platforms package-all github-release

help:
	@echo "Open Lake Controller $(VERSION) Build System"
	@echo "--------------------------------------------------"
	@echo "  make build          Build debug binary for host platform"
	@echo "  make release        Build optimized release binary for host"
	@echo "  make run            Run host application"
	@echo "  make screenshot     Run app and save verification screenshot"
	@echo "  make all-platforms  Build binaries for Windows, Linux, and macOS (using cross)"
	@echo "  make package-all    Bundle tar.gz / zip packages for all platforms into $(DIST_DIR)/"
	@echo "  make github-release Create GitHub release using GitHub CLI (gh)"
	@echo "  make clean          Clean build artifacts and distribution folder"

build:
	cargo build

release:
	cargo build $(RELEASE_FLAGS)

run:
	cargo run $(RELEASE_FLAGS)

screenshot: build
	cargo run -- --screenshot now.png

check:
	cargo check

clean:
	cargo clean
	rm -rf $(DIST_DIR) now.png test.png

# Multi-Platform Builds (uses 'cross' containerized toolchain if available, otherwise cargo)
build-windows:
	@echo "==> Building Windows x86_64 binary..."
	cargo build --release --target $(TARGET_WINDOWS_GNU) || cross build --release --target $(TARGET_WINDOWS_MSVC)

build-linux:
	@echo "==> Building Linux x86_64 binary..."
	cross build --release --target $(TARGET_LINUX) || cargo build --release --target $(TARGET_LINUX)

build-macos:
	@echo "==> Building macOS Universal/x86_64 & ARM binaries..."
	cross build --release --target $(TARGET_MACOS_X86) || true
	cross build --release --target $(TARGET_MACOS_ARM) || true

all-platforms: build-windows build-linux build-macos

# Packaging
package-all: all-platforms
	@echo "==> Packaging release distribution bundles..."
	mkdir -p $(DIST_DIR)
	# Windows bundle
	@if [ -f target/$(TARGET_WINDOWS_GNU)/release/$(APP_NAME).exe ]; then \
		zip -j $(DIST_DIR)/$(APP_NAME)-v$(VERSION)-windows-x86_64.zip target/$(TARGET_WINDOWS_GNU)/release/$(APP_NAME).exe README.md; \
	elif [ -f target/release/$(APP_NAME).exe ]; then \
		zip -j $(DIST_DIR)/$(APP_NAME)-v$(VERSION)-windows-x86_64.zip target/release/$(APP_NAME).exe README.md; \
	fi
	# Linux bundle
	@if [ -f target/$(TARGET_LINUX)/release/$(APP_NAME) ]; then \
		tar -czvf $(DIST_DIR)/$(APP_NAME)-v$(VERSION)-linux-x86_64.tar.gz -C target/$(TARGET_LINUX)/release $(APP_NAME); \
	fi
	# macOS bundles
	@if [ -f target/$(TARGET_MACOS_ARM)/release/$(APP_NAME) ]; then \
		tar -czvf $(DIST_DIR)/$(APP_NAME)-v$(VERSION)-macos-aarch64.tar.gz -C target/$(TARGET_MACOS_ARM)/release $(APP_NAME); \
	fi
	@if [ -f target/$(TARGET_MACOS_X86)/release/$(APP_NAME) ]; then \
		tar -czvf $(DIST_DIR)/$(APP_NAME)-v$(VERSION)-macos-x86_64.tar.gz -C target/$(TARGET_MACOS_X86)/release $(APP_NAME); \
	fi
	@echo "==> Packages generated in $(DIST_DIR)/:"
	@ls -la $(DIST_DIR) || dir $(DIST_DIR)

# GitHub Release Creation via gh CLI
github-release: package-all
	@echo "==> Publishing Release v$(VERSION) to GitHub..."
	gh release create v$(VERSION) $(DIST_DIR)/* \
		--title "Open Lake Controller v$(VERSION)" \
		--notes "Modern windowed recreation of Lake Controller v5.8 dedicated to Dolby Lake Processors." \
		--prerelease=false
