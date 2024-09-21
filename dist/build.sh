#!/bin/sh

platforms=(x86_64 arm64)

for platform in "${platforms[@]}"; do
  path="`nix-build --no-out-link -E '(import ./platforms.nix {}).bin'.$platform`"
  install "$path/bin/refold" "build/refold-$platform"

  path="`nix-build --no-out-link -E '(import ./platforms.nix {}).deb'.$platform`"
  cp "$path/refold.deb" "build/refold-$platform.deb"
  chmod +w "build/refold-$platform.deb"
done
