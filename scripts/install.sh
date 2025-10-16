#!/bin/bash
set -e

# RavensOne Installation Script

REPO="yourusername/ravensone"
BINARY_NAME="raven"
INSTALL_DIR="${HOME}/.ravensone/bin"

echo "🚀 Installing RavensOne..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${ARCH}" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)
        echo "❌ Unsupported architecture: ${ARCH}"
        exit 1
        ;;
esac

# Download latest release
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}-${OS}-${ARCH}"

echo "📥 Downloading from ${DOWNLOAD_URL}..."

# Create install directory
mkdir -p "${INSTALL_DIR}"

# Download binary
curl -L "${DOWNLOAD_URL}" -o "${INSTALL_DIR}/${BINARY_NAME}"

# Make executable
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Add to PATH
SHELL_CONFIG="${HOME}/.bashrc"
if [ -f "${HOME}/.zshrc" ]; then
    SHELL_CONFIG="${HOME}/.zshrc"
fi

if ! grep -q ".ravensone/bin" "${SHELL_CONFIG}"; then
    echo "" >> "${SHELL_CONFIG}"
    echo "# RavensOne" >> "${SHELL_CONFIG}"
    echo "export PATH=\"\${HOME}/.ravensone/bin:\${PATH}\"" >> "${SHELL_CONFIG}"
    echo "✓ Added to PATH in ${SHELL_CONFIG}"
fi

echo "✨ RavensOne installed successfully!"
echo ""
echo "Run 'source ${SHELL_CONFIG}' or restart your terminal, then try:"
echo "  raven --version"
echo "  raven init my-app"
echo ""
echo "📖 Documentation: https://ravensone.dev/docs"
