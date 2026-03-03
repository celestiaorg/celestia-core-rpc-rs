#!/usr/bin/env bash
set -euo pipefail

REPO=${REPO:-celestiaorg/celestia-core-rpc-rs}
BIN=${BIN:-celestiacore}
VERSION=${VERSION:-latest}
PREFIX=${PREFIX:-/usr/local}
TARGET=${TARGET:-}

if [ -z "$TARGET" ]; then
  uname_s=$(uname -s)
  uname_m=$(uname -m)

  case "$uname_s" in
    Linux) os="unknown-linux-gnu" ;;
    Darwin) os="apple-darwin" ;;
    *)
      echo "Unsupported OS: $uname_s" >&2
      exit 1
      ;;
  esac

  case "$uname_m" in
    x86_64|amd64) arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *)
      echo "Unsupported arch: $uname_m" >&2
      exit 1
      ;;
  esac

  TARGET="${arch}-${os}"
fi

if [ "$VERSION" = "latest" ]; then
  api_url="https://api.github.com/repos/${REPO}/releases/latest"
  tag=$(curl -sSfL "$api_url" | grep -m1 '"tag_name"' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
else
  tag="$VERSION"
fi

asset="${BIN}-${tag}-${TARGET}.tar.gz"
download_url="https://github.com/${REPO}/releases/download/${tag}/${asset}"

tmpdir=$(mktemp -d)
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT

echo "Downloading ${download_url}"
curl -sSfL "$download_url" -o "${tmpdir}/${asset}"

tar -xzf "${tmpdir}/${asset}" -C "$tmpdir"

install_dir="${PREFIX}/bin"
mkdir -p "$install_dir"
install -m 755 "${tmpdir}/${BIN}" "${install_dir}/${BIN}"

echo "Installed ${BIN} to ${install_dir}/${BIN}"
echo "Make sure ${install_dir} is on your PATH"
