#!/bin/sh
set -eu

limo_release_base_url=${LIMO_RELEASE_BASE_URL:-https://github.com/Spxg/limo/releases/latest/download}
limo_install_dir=${LIMO_INSTALL_DIR:-${HOME:?HOME is not set}/.local/bin}
limo_tmp_dir=
limo_staged_binary=

limo_fail() {
    printf 'limo installer: %s\n' "$*" >&2
    exit 1
}

limo_cleanup() {
    if [ -n "$limo_staged_binary" ]; then
        rm -f "$limo_staged_binary"
    fi
    if [ -n "$limo_tmp_dir" ]; then
        rm -rf "$limo_tmp_dir"
    fi
}

trap limo_cleanup 0
trap 'exit 1' HUP INT TERM

limo_system=$(uname -s)
limo_machine=$(uname -m)
case "$limo_system:$limo_machine" in
    Linux:x86_64 | Linux:amd64) limo_target=x86_64-unknown-linux-gnu ;;
    Linux:arm64 | Linux:aarch64) limo_target=aarch64-unknown-linux-gnu ;;
    Darwin:x86_64 | Darwin:amd64) limo_target=x86_64-apple-darwin ;;
    Darwin:arm64 | Darwin:aarch64) limo_target=aarch64-apple-darwin ;;
    *) limo_fail "unsupported platform: $limo_system $limo_machine" ;;
esac

for limo_command in curl tar mktemp install; do
    command -v "$limo_command" >/dev/null 2>&1 || limo_fail "$limo_command is required"
done

limo_archive_name="limo-$limo_target.tar.gz"
limo_release_base_url=${limo_release_base_url%/}
limo_tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/limo-install.XXXXXX")
limo_archive_path="$limo_tmp_dir/$limo_archive_name"
limo_download_url="$limo_release_base_url/$limo_archive_name"

printf 'Downloading %s...\n' "$limo_archive_name"
case "$limo_download_url" in
    https://*)
        curl --proto '=https' --tlsv1.2 -fsSL "$limo_download_url" -o "$limo_archive_path" ||
            limo_fail "could not download $limo_download_url"
        ;;
    *)
        curl -fsSL "$limo_download_url" -o "$limo_archive_path" ||
            limo_fail "could not download $limo_download_url"
        ;;
esac

tar -xzf "$limo_archive_path" -C "$limo_tmp_dir" "limo-$limo_target/limo" ||
    limo_fail "could not extract limo from $limo_archive_name"
limo_binary="$limo_tmp_dir/limo-$limo_target/limo"
if [ ! -f "$limo_binary" ] || [ -L "$limo_binary" ]; then
    limo_fail "downloaded archive does not contain a regular limo executable"
fi

mkdir -p "$limo_install_dir"
[ ! -d "$limo_install_dir/limo" ] || limo_fail "$limo_install_dir/limo is a directory"
limo_staged_binary=$(mktemp "$limo_install_dir/.limo.XXXXXX")
install -m 0755 "$limo_binary" "$limo_staged_binary"
mv -f "$limo_staged_binary" "$limo_install_dir/limo"
limo_staged_binary=
printf 'Installed limo to %s/limo\n' "$limo_install_dir"

case ":${PATH:-}:" in
    *":$limo_install_dir:"*) ;;
    *) printf 'Add %s to PATH, then run: limo --version\n' "$limo_install_dir" ;;
esac
