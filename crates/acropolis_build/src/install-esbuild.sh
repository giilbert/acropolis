#!/bin/sh
set -eu
dir=$(mktemp -d)
platform=$(uname -ms)
tgz="$dir/esbuild-0.18.11.tgz"

# Download the binary executable for the current platform
case $platform in
  'Darwin arm64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/darwin-arm64/-/darwin-arm64-0.18.11.tgz";;
  'Darwin x86_64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/darwin-x64/-/darwin-x64-0.18.11.tgz";;
  'Linux arm64' | 'Linux aarch64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/linux-arm64/-/linux-arm64-0.18.11.tgz";;
  'Linux x86_64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.18.11.tgz";;
  'NetBSD amd64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/netbsd-x64/-/netbsd-x64-0.18.11.tgz";;
  'OpenBSD amd64') curl -sfo "$tgz" "https://registry.npmjs.org/@esbuild/openbsd-x64/-/openbsd-x64-0.18.11.tgz";;
  *) echo "error: Unsupported platform: $platform"; exit 1
esac

# Extract the binary executable to the current directory
tar -xzf "$tgz" -C "$dir" package/bin/esbuild
mv "$dir/package/bin/esbuild" .
rm "$tgz"
