#!/bin/bash
# Build Script für inoX-MIX Release

set -e

echo "================================"
echo "inoX-MIX Release Build"
echo "================================"

# Versionen prüfen
echo ""
echo "Checking versions..."
CARGO_VERSION=$(grep '^version' src-tauri/Cargo.toml | head -1 | cut -d'"' -f2)
PACKAGE_VERSION=$(grep '"version"' package.json | head -1 | cut -d'"' -f4)
TAURI_VERSION=$(grep '"version"' src-tauri/tauri.conf.json | head -1 | cut -d'"' -f4)

echo "  Cargo.toml:        $CARGO_VERSION"
echo "  package.json:      $PACKAGE_VERSION"
echo "  tauri.conf.json:   $TAURI_VERSION"

if [ "$CARGO_VERSION" != "$PACKAGE_VERSION" ] || [ "$CARGO_VERSION" != "$TAURI_VERSION" ]; then
    echo ""
    echo "❌ ERROR: Version mismatch!"
    echo "   Please sync versions across all config files."
    exit 1
fi

echo "✓ Versions match: $CARGO_VERSION"

# Dependencies prüfen
echo ""
echo "Checking dependencies..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js not found"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "❌ npm not found"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Cargo not found"; exit 1; }

NODE_VERSION=$(node --version)
CARGO_VERSION_FULL=$(cargo --version)
echo "✓ Node.js: $NODE_VERSION"
echo "✓ Cargo:   $CARGO_VERSION_FULL"

# Clean vorheriger Build
echo ""
echo "Cleaning previous builds..."
rm -rf dist/
rm -rf src-tauri/target/release/bundle/
echo "✓ Clean complete"

# Frontend installieren
echo ""
echo "Installing frontend dependencies..."
npm install
echo "✓ Frontend dependencies installed"

# Frontend bauen
echo ""
echo "Building frontend..."
npm run build
echo "✓ Frontend build complete"

# Backend bauen
echo ""
echo "Building Tauri app..."
npm run tauri build

# Output prüfen
echo ""
echo "================================"
echo "Build Summary"
echo "================================"

BUNDLE_DIR="src-tauri/target/release/bundle"

if [ -d "$BUNDLE_DIR/appimage" ]; then
    echo ""
    echo "AppImage:"
    ls -lh "$BUNDLE_DIR/appimage/"*.AppImage 2>/dev/null || echo "  No AppImage found"
fi

if [ -d "$BUNDLE_DIR/deb" ]; then
    echo ""
    echo "Debian Package:"
    ls -lh "$BUNDLE_DIR/deb/"*.deb 2>/dev/null || echo "  No .deb found"
fi

echo ""
echo "Binary:"
ls -lh "src-tauri/target/release/inox-mix" 2>/dev/null || echo "  No binary found"

echo ""
echo "================================"
echo "✓ Build complete!"
echo "================================"
echo ""
echo "Next steps:"
echo "  1. Test the binary: ./src-tauri/target/release/inox-mix"
echo "  2. Test AppImage:   ./src-tauri/target/release/bundle/appimage/*.AppImage"
echo "  3. Create git tag:  git tag v$CARGO_VERSION"
echo "  4. Push tag:        git push origin v$CARGO_VERSION"
echo ""
