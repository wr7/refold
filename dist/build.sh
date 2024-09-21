#!/bin/sh

platforms=(x86_64 arm64)
package_types=(deb apk rpm)

for platform in "${platforms[@]}"; do
  path="`nix-build --no-out-link -E '(import ./platforms.nix {}).bin'.$platform`"
  install "$path/bin/refold" "build/refold-$platform"

  for type in "${package_types[@]}"; do
    path="`nix-build --no-out-link -E '(import ./platforms.nix {})'.$type.$platform`"
    cp "$path/refold.$type" "build/refold-$platform.$type"
    chmod +w "build/refold-$platform.$type"
  done
done
