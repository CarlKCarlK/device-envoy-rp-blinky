#!/usr/bin/env bash
set -euo pipefail

action="${1:-}"
arg1="${2:-}"
arg2="${3:-}"

board="1"
if [[ "$arg1" == "blinky" ]]; then
  board="${arg2:-1}"
elif [[ -n "$arg1" ]]; then
  board="$arg1"
fi

if [[ -z "$action" ]]; then
  echo "usage: scripts/device-action.sh <run|check|build> [blinky] [board]" >&2
  exit 1
fi

case "$action" in
  run|check|build) ;;
  *)
    echo "invalid action '$action' (expected: run, check, build)" >&2
    exit 1
    ;;
esac

target=""
features=""
case "$board" in
  1)
    target="thumbv6m-none-eabi"
    features="pico1"
    ;;
  2)
    target="thumbv8m.main-none-eabihf"
    features="pico2"
    ;;
  w)
    target="thumbv6m-none-eabi"
    features="pico1,wifi"
    ;;
  2w)
    target="thumbv8m.main-none-eabihf"
    features="pico2,wifi"
    ;;
  *)
    echo "invalid board '$board' (expected one of: 1, 2, w, 2w)" >&2
    exit 1
    ;;
esac

release_args=()
if [[ "$action" != "check" ]]; then
  release_args=(--release)
fi

cargo "$action" \
  --target "$target" \
  "${release_args[@]}" \
  --no-default-features \
  --features "$features"
