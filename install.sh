#!/bin/sh
# AgentGen CLI installer
# Usage: curl -fsSL https://raw.githubusercontent.com/Agent-Gen-com/agent-gen-lib/main/install.sh | sh

set -e

REPO="Agent-Gen-com/agent-gen-lib"
BINARY="agentgen"
INSTALL_DIR="${AGENTGEN_INSTALL:-$HOME/.local/bin}"

# Detect OS
case "$(uname -s)" in
  Darwin) OS="macos" ;;
  Linux)  OS="linux" ;;
  *)
    echo "Unsupported OS: $(uname -s)" >&2
    exit 1
    ;;
esac

# Detect architecture
case "$(uname -m)" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

# Resolve version
if [ -z "$AGENTGEN_VERSION" ]; then
  echo "Fetching latest release..."
  if command -v curl >/dev/null 2>&1; then
    VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | grep '"tag_name"' | sed 's/.*"tag_name": *"//;s/".*//')
  elif command -v wget >/dev/null 2>&1; then
    VERSION=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" \
      | grep '"tag_name"' | sed 's/.*"tag_name": *"//;s/".*//')
  else
    echo "Error: curl or wget is required" >&2
    exit 1
  fi
  if [ -z "$VERSION" ]; then
    echo "Error: could not determine latest release version" >&2
    exit 1
  fi
else
  VERSION="$AGENTGEN_VERSION"
fi

TARBALL="${BINARY}-${ARCH}-${OS}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${TARBALL}"

echo "Installing agentgen ${VERSION} (${ARCH}-${OS})..."

# Download
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$URL" -o "$TMP_DIR/$TARBALL"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$TMP_DIR/$TARBALL" "$URL"
fi

# Extract
tar xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"

# Install
mkdir -p "$INSTALL_DIR"
mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"

echo "Installed to $INSTALL_DIR/$BINARY"

# PATH advice
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo ""
    echo "NOTE: $INSTALL_DIR is not in your PATH."
    echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    ;;
esac

echo ""
echo "Next steps:"
echo "  1. Set your API key: export AGENTGEN_API_KEY=your_key_here"
echo "  2. Run: agentgen --help"
echo ""
echo "Get your API key at: https://www.agent-gen.com"
