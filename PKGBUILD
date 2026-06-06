# Maintainer: KiyokoDev <kiyoko@kiyoko.dev>

pkgname=Markup-Swift
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimal Markdown editor built with egui and Rust"
arch=('x86_64')
url="https://github.com/KiyokoDev/Markup-Swift"
license=('LicenseRef-custom')
depends=('gcc-libs' 'fontconfig')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/KiyokoDev/Markup-Swift/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    RUSTFLAGS="-C link-arg=-s" cargo build --release --frozen
}

package() {
    cd "$srcdir/$pkgname-$pkgver"

    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "resources/$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
    install -Dm644 "resources/$pkgname.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/$pkgname.svg"

    install -Dm644 "resources/fonts/Lexend-Regular.ttf" \
        "$pkgdir/usr/share/fonts/TTF/Lexend-Regular.ttf"
    install -Dm644 "resources/fonts/Lexend-Bold.ttf" \
        "$pkgdir/usr/share/fonts/TTF/Lexend-Bold.ttf"
    install -Dm644 "resources/fonts/JetBrainsMono-Regular.ttf" \
        "$pkgdir/usr/share/fonts/TTF/JetBrainsMono-Regular.ttf"
    install -Dm644 "resources/fonts/JetBrainsMono-Bold.ttf" \
        "$pkgdir/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf"
    install -Dm644 "resources/fonts/JetBrainsMono-Italic.ttf" \
        "$pkgdir/usr/share/fonts/TTF/JetBrainsMono-Italic.ttf"
    install -Dm644 "resources/fonts/JetBrainsMono-BoldItalic.ttf" \
        "$pkgdir/usr/share/fonts/TTF/JetBrainsMono-BoldItalic.ttf"

    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
