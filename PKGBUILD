# Maintainer: Kairat Kubanychbek uulu <https://github.com/ImKairat>

pkgname=gitflow
pkgver=1.0.0_beta.1
pkgrel=1
pkgdesc="Gitflow is a Git extension that implements the Gitflow AVH branching model."
arch=('x86_64' 'aarch64')
url="https://github.com/ImKairat-Rust-Lab/gitflow"
license=('GPL-3.0-only')
depends=('gcc-libs')
makedepends=('cargo')
options=('!lto')
source=()
sha256sums=()

build() {
  cd "${startdir}"
  cargo build --release
}

package() {
  install -Dm755 "${startdir}/target/release/gitflow" "${pkgdir}/usr/bin/gitflow"
}
