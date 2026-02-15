#!/bin/bash
# inoX-MIX AUR Package Test-Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔨 inoX-MIX AUR Package Test"
echo "============================"
echo ""

# Prüfe ob makepkg verfügbar ist
if ! command -v makepkg &> /dev/null; then
    echo "❌ makepkg ist nicht installiert!"
    echo "   Dies ist kein Arch Linux System oder base-devel fehlt."
    echo "   Install: sudo pacman -S base-devel"
    exit 1
fi

# Prüfe ob namcap installiert ist (optional)
if command -v namcap &> /dev/null; then
    HAVE_NAMCAP=1
else
    echo "⚠️  namcap nicht gefunden (optional für Validierung)"
    echo "   Install: sudo pacman -S namcap"
    HAVE_NAMCAP=0
fi

echo ""
echo "📋 Schritt 1: PKGBUILD validieren"
if [ $HAVE_NAMCAP -eq 1 ]; then
    namcap PKGBUILD || echo "⚠️  Namcap-Warnungen (oft ignorierbar)"
else
    echo "⏭️  Übersprungen (namcap nicht installiert)"
fi

echo ""
echo "📋 Schritt 2: .SRCINFO generieren"
makepkg --printsrcinfo > .SRCINFO
echo "✅ .SRCINFO erstellt"

echo ""
echo "📋 Schritt 3: Dependencies prüfen"
echo "Benötigte Pakete:"
echo "  - rust cargo nodejs npm git (makedepends)"
echo "  - pipewire webkit2gtk gtk3 libsoup javascriptcore (depends)"

missing_deps=0
for dep in rust cargo nodejs npm git pipewire webkit2gtk gtk3 libsoup javascriptcore; do
    if ! pacman -Q $dep &> /dev/null; then
        echo "❌ $dep fehlt"
        missing_deps=1
    fi
done

if [ $missing_deps -eq 1 ]; then
    echo ""
    echo "⚠️  Fehlende Dependencies gefunden!"
    read -p "Installieren? [y/N]: " install_deps
    if [ "$install_deps" = "y" ] || [ "$install_deps" = "Y" ]; then
        sudo pacman -S --needed rust cargo nodejs npm git pipewire webkit2gtk gtk3 libsoup javascriptcore
    else
        echo "❌ Abbruch - Dependencies fehlen"
        exit 1
    fi
fi

echo ""
echo "📋 Schritt 4: Test-Build"
echo "Wähle Build-Modus:"
echo "  1) Schneller Build (überspringt Tests)"
echo "  2) Vollständiger Build (mit Tests)"
echo "  3) Clean-Chroot Build (wie AUR-Builder)"
echo ""
read -p "Auswahl [1]: " choice
choice=${choice:-1}

case $choice in
    1)
        echo ""
        echo "🔨 Starte schnellen Build..."
        makepkg -f --skipinteg
        ;;

    2)
        echo ""
        echo "🔨 Starte vollständigen Build..."
        makepkg -f
        ;;

    3)
        if ! command -v extra-x86_64-build &> /dev/null; then
            echo "❌ extra-x86_64-build nicht gefunden!"
            echo "   Install: sudo pacman -S devtools"
            exit 1
        fi
        echo ""
        echo "🔨 Starte Clean-Chroot Build..."
        extra-x86_64-build
        ;;

    *)
        echo "❌ Ungültige Auswahl"
        exit 1
        ;;
esac

echo ""
echo "📋 Schritt 5: Paket validieren"
PKG_FILE=$(ls inox-mix-*.pkg.tar.zst 2>/dev/null | head -1)

if [ -z "$PKG_FILE" ]; then
    echo "❌ Kein Paket gefunden!"
    exit 1
fi

echo "Gefundenes Paket: $PKG_FILE"

if [ $HAVE_NAMCAP -eq 1 ]; then
    namcap "$PKG_FILE" || echo "⚠️  Namcap-Warnungen (oft ignorierbar)"
else
    echo "⏭️  Namcap-Validierung übersprungen"
fi

echo ""
echo "✅ Build erfolgreich!"
echo ""
echo "📦 Paket: $PKG_FILE"
echo ""
echo "🚀 Nächste Schritte:"
echo "   1) Paket installieren: sudo pacman -U $PKG_FILE"
echo "   2) App testen: inox-mix"
echo "   3) Bei Erfolg: AUR hochladen"
echo ""

read -p "Paket jetzt installieren? [y/N]: " install_now
if [ "$install_now" = "y" ] || [ "$install_now" = "Y" ]; then
    sudo pacman -U "$PKG_FILE"
    echo ""
    echo "✅ Installiert! Starte mit: inox-mix"
fi
