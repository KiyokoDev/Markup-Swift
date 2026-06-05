#!/usr/bin/env bash
set -euo pipefail

APP="markup-swift"
ARCH="x86_64"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── Download fonts if missing ─────────────────────────────────────────
FONT_DIR="$SCRIPT_DIR/resources/fonts"
if [ ! -f "$FONT_DIR/Lexend-Regular.ttf" ]; then
    echo "==> Downloading fonts …"
    mkdir -p "$FONT_DIR"
    LEXEND="https://raw.githubusercontent.com/googlefonts/lexend/main/fonts/lexend/ttf"
    JETBRAINS="https://raw.githubusercontent.com/JetBrains/JetBrainsMono/master/fonts/ttf"
    curl -fsSL "$LEXEND/Lexend-Regular.ttf"   -o "$FONT_DIR/Lexend-Regular.ttf" &
    curl -fsSL "$LEXEND/Lexend-Bold.ttf"      -o "$FONT_DIR/Lexend-Bold.ttf" &
    curl -fsSL "$JETBRAINS/JetBrainsMono-Regular.ttf" -o "$FONT_DIR/JetBrainsMono-Regular.ttf" &
    curl -fsSL "$JETBRAINS/JetBrainsMono-Bold.ttf"    -o "$FONT_DIR/JetBrainsMono-Bold.ttf" &
    curl -fsSL "$JETBRAINS/JetBrainsMono-Italic.ttf"  -o "$FONT_DIR/JetBrainsMono-Italic.ttf" &
    curl -fsSL "$JETBRAINS/JetBrainsMono-BoldItalic.ttf" -o "$FONT_DIR/JetBrainsMono-BoldItalic.ttf" &
    wait
    echo "   Fonts downloaded"
fi

# ── Detect target ────────────────────────────────────────────────────
BUILD_WIN=false
for arg in "$@"; do
    case "$arg" in
        --win) BUILD_WIN=true ;;
        --help)
            echo "Usage: $0 [--win]"
            echo ""
            echo "  (no flag)   Build Linux AppImage (default)"
            echo "  --win       Cross-compile Windows .exe"
            exit 0
            ;;
    esac
done

if $BUILD_WIN; then
    # ══════════════════════════════════════════════════════════════════
    #  Windows .exe build
    # ══════════════════════════════════════════════════════════════════
    TARGET="x86_64-pc-windows-gnu"
    echo "==> Building $APP for Windows ($TARGET)"

    rustup target add "$TARGET" 2>/dev/null || true

    RUSTFLAGS="-C link-arg=-s" cargo build --release --target "$TARGET"

    BIN="target/$TARGET/release/${APP}.exe"
    if [ ! -f "$BIN" ]; then
        echo "!! Build failed – $BIN not found" >&2
        exit 1
    fi
    echo "   Binary: $BIN ($(du -h "$BIN" | cut -f1))"

    OUTDIR="$SCRIPT_DIR/target/${APP}-win64"
    rm -rf "$OUTDIR"
    mkdir -p "$OUTDIR"

    cp "$BIN" "$OUTDIR/"
    cp "resources/$APP.svg" "$OUTDIR/"
    cp "README.md" "$OUTDIR/" 2>/dev/null || true

    ZIP="$SCRIPT_DIR/target/${APP}-${ARCH}-win64.zip"
    rm -f "$ZIP"
    (cd "$OUTDIR" && zip -r "$ZIP" .)

    echo ""
    echo "==> Done: $ZIP ($(du -h "$ZIP" | cut -f1))"
    echo "==> Extract and run: ${APP}.exe"
    exit 0
fi

# ══════════════════════════════════════════════════════════════════════
#  Linux AppImage build
# ══════════════════════════════════════════════════════════════════════
echo "==> Building $APP (Markdown editor)"
echo ""

# ── 1. Build binary ──────────────────────────────────────────────
RUSTFLAGS="-C link-arg=-s" cargo build --release

BIN="target/release/$APP"
if [ ! -f "$BIN" ]; then
    echo "!! Build failed – binary not found" >&2
    exit 1
fi
echo "   Binary: $BIN ($(du -h "$BIN" | cut -f1))"

# ── 2. Create AppDir ─────────────────────────────────────────────
APPDIR="$SCRIPT_DIR/target/${APP}.AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"

cp "$BIN" "$APPDIR/usr/bin/$APP"
cp "resources/$APP.desktop" "$APPDIR/usr/share/applications/"
cp "resources/$APP.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/"
cp "resources/$APP.svg" "$APPDIR/"

# Desktop file in root (required by AppImage)
cp "resources/$APP.desktop" "$APPDIR/"
sed -i "s|^Exec=.*|Exec=$APP|" "$APPDIR/$APP.desktop"

# ── 3. AppRun entry point ────────────────────────────────────────
cat > "$APPDIR/AppRun" << 'EOF'
#!/usr/bin/env bash
HERE="$(cd "$(dirname "$0")" && pwd)"
export PATH="$HERE/usr/bin:$PATH"
export XDG_DATA_DIRS="$HERE/usr/share:$XDG_DATA_DIRS"
exec "$HERE/usr/bin/$APP" "$@"
EOF
chmod +x "$APPDIR/AppRun"

echo "   AppDir: $APPDIR"

# ── 4. Download appimagetool if missing ───────────────────────────
TOOL="$SCRIPT_DIR/target/appimagetool"
if [ ! -f "$TOOL" ]; then
    echo "   Downloading appimagetool …"
    URL="https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-${ARCH}.AppImage"
    if command -v curl &>/dev/null; then
        curl -fsSL "$URL" -o "$TOOL"
    elif command -v wget &>/dev/null; then
        wget -q "$URL" -O "$TOOL"
    else
        echo "!! Need curl or wget to download appimagetool" >&2
        exit 1
    fi
    chmod +x "$TOOL"
fi

# ── 5. Generate .AppImage ────────────────────────────────────────
OUTPUT="$SCRIPT_DIR/target/${APP}-${ARCH}.AppImage"
rm -f "$OUTPUT"

if "$TOOL" "$APPDIR" "$OUTPUT" &>/dev/null; then
    :
else
    "$TOOL" --no-appimage-extract "$APPDIR" "$OUTPUT" 2>/dev/null || \
    "$TOOL" "$APPDIR" "$OUTPUT" 2>/dev/null
fi

if [ -f "$OUTPUT" ]; then
    echo ""
    echo "==> Done: $OUTPUT ($(du -h "$OUTPUT" | cut -f1))"
    echo "==> Run:  $OUTPUT"
else
    echo "!! AppImage creation failed" >&2
    exit 1
fi
