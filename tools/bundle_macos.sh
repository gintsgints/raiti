#!/usr/bin/env bash
# Wraps a built binary in Raiti.app and packs it into a disk image.
#
#   tools/bundle_macos.sh target/release/raiti [output directory]
#
# Signing, when there is a certificate to sign with, goes between the two:
# codesign the finished .app, notarize it, staple the ticket, then build the
# disk image from the stapled bundle.
set -euo pipefail

binary="${1:?usage: bundle_macos.sh <binary> [output directory]}"
out_dir="${2:-target/macos}"

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"

app="$out_dir/Raiti.app"
rm -rf "$app" "$out_dir/dmg"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

cp "$binary" "$app/Contents/MacOS/raiti"
chmod +x "$app/Contents/MacOS/raiti"
cp "$root/macos/raiti.icns" "$app/Contents/Resources/raiti.icns"
sed "s/VERSION/$version/g" "$root/macos/Info.plist" > "$app/Contents/Info.plist"

# An unsigned binary still needs an ad-hoc signature to run on Apple silicon,
# and the bundle must be signed as a whole for the icon and identity to stick.
codesign --force --sign - "$app"

# Disk image with the usual drag-to-install layout.
mkdir -p "$out_dir/dmg"
cp -R "$app" "$out_dir/dmg/"
ln -s /Applications "$out_dir/dmg/Applications"
hdiutil create \
	-volname "Raiti $version" \
	-srcfolder "$out_dir/dmg" \
	-ov -format UDZO \
	"$out_dir/raiti-aarch64-apple-darwin.dmg"
rm -rf "$out_dir/dmg"

echo "built $app and $out_dir/raiti-aarch64-apple-darwin.dmg"
