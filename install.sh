#!/usr/bin/env bash
set -euo pipefail

SOURCE="${BASH_SOURCE[0]}"
while [ -L "$SOURCE" ]; do
  APP_PATH="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$APP_PATH/$SOURCE"
done
APP_PATH="$( cd -P "$( dirname "$SOURCE" )" && pwd )"

BIN="$APP_PATH/target/release/dotfiles-installer"
if [ -x "$BIN" ]; then
  exec "$BIN" "$@"
fi

if command -v cargo >/dev/null 2>&1; then
  exec cargo run --quiet --manifest-path "$APP_PATH/Cargo.toml" -- "$@"
fi

echo "ERR! missing dotfiles installer binary and cargo"
echo "Hint: install Rust toolchain, then run: cargo build --release"
exit 1
