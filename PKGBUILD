# Maintainer: matteac <mateoeestn2@gmail.com>
pkgname=rat
pkgver=0.1.r10.b0100aa
pkgrel=1
epoch=
pkgdesc="cat written in rust"
arch=(x86_64)
url="https://github.com/matteac/rat"
license=('MIT')
groups=()
depends=()
makedepends=(cargo git)
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+$url")
noextract=()
md5sums=("SKIP")
validpgpkeys=()



pkgver() {
	cd "${_pkgname}"
    printf "0.1.r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
    cd rat
	make
}

package() {
    cd "${_pkgname}"
    mkdir -p ${pkgdir}/opt/${pkgname}
    cp -rf * ${pkgdir}/opt/${pkgname}

	make install
    install -Dm644 README.md "${pkgdir}/usr/share/doc/${pkgname}/README.md"
}
