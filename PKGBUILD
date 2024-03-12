# Mantainer: Adrián Nicolás <nicolas.aguilera.adrian@gmail.com>

pkgname=hypr-workspaces
pkgver=0.1.0
pkgrel=1
pkgdesc="Allows monitors to swap workspaces in hyprland"
arch=('x86_64')
url="https://github.com/adriannic/hypr-workspaces"
makedepends=('rust')
source=()

build() {
  cd "$startdir"
  cargo build --release --locked
}

package () {
  cd "$startdir"
  install -Dm755 target/release/hypr-workspaces "$pkgdir"/usr/bin/hypr-workspaces
}
