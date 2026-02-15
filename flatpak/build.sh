#!/bin/bash
# inoX-MIX Flatpak Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔨 inoX-MIX Flatpak Builder"
echo "=========================="
echo ""

# Prüfe ob flatpak-builder installiert ist
if ! command -v flatpak-builder &> /dev/null; then
    echo "❌ flatpak-builder ist nicht installiert!"
    echo "   Install: sudo pacman -S flatpak-builder"
    exit 1
fi

# Prüfe ob SDKs installiert sind
echo "📦 Prüfe Flatpak SDKs..."
if ! flatpak list | grep -q "org.freedesktop.Sdk/x86_64/23.08"; then
    echo "⚠️  Freedesktop SDK 23.08 nicht gefunden. Installiere..."
    flatpak install -y flathub org.freedesktop.Platform//23.08
    flatpak install -y flathub org.freedesktop.Sdk//23.08
    flatpak install -y flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
    flatpak install -y flathub org.freedesktop.Sdk.Extension.node18//23.08
fi

echo "✅ SDKs vorhanden"
echo ""

# Build-Modus wählen
echo "Wähle Build-Modus:"
echo "  1) Lokaler Build + Install (schnell, zum Testen)"
echo "  2) Bundle erstellen (.flatpak für Distribution)"
echo ""
read -p "Auswahl [1]: " choice
choice=${choice:-1}

case $choice in
    1)
        echo ""
        echo "🔨 Starte lokalen Build..."
        flatpak-builder --force-clean --user --install --install-deps-from=flathub \
            build-dir network.inox.mix.yml

        echo ""
        echo "✅ Build abgeschlossen!"
        echo ""
        echo "🚀 App starten mit:"
        echo "   flatpak run network.inox.mix"
        ;;

    2)
        echo ""
        echo "🔨 Erstelle Bundle..."

        # Bauen und Repository erstellen
        flatpak-builder --force-clean --repo=repo build-dir network.inox.mix.yml

        # Bundle exportieren
        VERSION=$(grep 'version:' network.inox.mix.yml | head -1 | awk '{print $2}' | tr -d '"')
        BUNDLE_NAME="inox-mix-${VERSION}.flatpak"

        flatpak build-bundle repo "$BUNDLE_NAME" network.inox.mix

        echo ""
        echo "✅ Bundle erstellt: $BUNDLE_NAME"
        echo ""
        echo "📦 Bundle installieren mit:"
        echo "   flatpak install $BUNDLE_NAME"
        ;;

    *)
        echo "❌ Ungültige Auswahl"
        exit 1
        ;;
esac
