#!/bin/sh
# Puts the 333 client on this machine, and does nothing else.
#
#   curl -fsSL https://the333.dev/install.sh | sh
#   curl -fsSL https://the333.dev/install.sh | sh -s -- --light
#
# It works out which file fits this machine, fetches that file and the release's SHA256SUMS
# from github.com/needmoretruth/333, refuses unless the two agree, and puts the file at
# ~/.local/bin/333. It starts nothing, installs no service, asks for no password and touches
# no file outside ~/.local/bin. Read it before you run it; it is short on purpose.
#
#   --light   the Light form: the same client without the terminal screen
#   --help    this, and nothing else
#
# FOR TESTING ONLY: THE333_RELEASE_BASE replaces the release address, so that this script
# can be run against a directory served on this machine. Nobody installing needs it.

set -eu

release_base=${THE333_RELEASE_BASE:-https://github.com/needmoretruth/333/releases/latest/download}
# Only the real address is held to https. A test base is a server on this machine.
if [ -n "${THE333_RELEASE_BASE:-}" ]; then proto='=http,https'; else proto='=https'; fi

say() { printf '%s\n' "$*"; }
fail() {
  printf '%s\n' "$@" >&2
  exit 1
}

usage() {
  say "usage: install.sh [--light]"
  say ""
  say "Fetches the 333 client for this machine from the latest release, checks it against"
  say "that release's SHA256SUMS, and puts it at ~/.local/bin/333. It starts nothing."
  say "--light takes the Light form, which is the same client without the terminal screen."
}

light=""
for arg in "$@"; do
  case "$arg" in
    --light) light="light-" ;;
    -h | --help) usage; exit 0 ;;
    *) fail "refused  '$arg' is not something this script understands. It takes --light or nothing." ;;
  esac
done

[ -n "${HOME:-}" ] || fail "refused  HOME is not set, so there is no ~/.local/bin to put the file in."

# Which file fits. The names are the ones the release attaches, and nothing else is built.
system=$(uname -s)
machine=$(uname -m)
case "$system" in
  Linux)
    os=linux
    case "$machine" in
      x86_64 | amd64) arch=x86_64 ;;
      aarch64 | arm64) arch=aarch64 ;;
      armv6* | armv7* | armv8l | armhf | arm) arch=armv6 ;;
      *) fail "refused  No file is built for Linux on $machine." \
        "         The repository says how to build it yourself: https://github.com/needmoretruth/333#install" ;;
    esac
    # A 64-bit kernel under a 32-bit system, which is what 32-bit Raspberry Pi OS on a Pi 4
    # is: the kernel says aarch64 and nothing on the disk can run a 64-bit program.
    if [ "$arch" = aarch64 ] && [ "$(getconf LONG_BIT 2>/dev/null || echo 64)" = 32 ]; then
      arch=armv6
    fi
    # The files are built against the GNU C library. On a system built on musl, Alpine among
    # them, they would be installed and then fail to start with a message about a missing file.
    if (ldd --version 2>&1 || true) | grep -qi musl; then
      fail "refused  This system uses musl rather than the GNU C library, and the files are built" \
        "         for the second. The repository says how to build it yourself:" \
        "         https://github.com/needmoretruth/333#install"
    fi
    ;;
  Darwin)
    os=macos
    case "$machine" in
      arm64 | aarch64) arch=aarch64 ;;
      x86_64)
        arch=x86_64
        # A shell running under Rosetta on Apple silicon says x86_64. The chip is still arm64,
        # and the arm64 file is the one that runs without a translator in between.
        if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)" = 1 ]; then arch=aarch64; fi
        ;;
      *) fail "refused  No file is built for macOS on $machine." ;;
    esac
    ;;
  MINGW* | MSYS* | CYGWIN*)
    fail "refused  This is Windows. In PowerShell, run:" \
      "         irm https://the333.dev/install.ps1 | iex"
    ;;
  *)
    fail "refused  No file is built for $system." \
      "         The repository says how to build it yourself: https://github.com/needmoretruth/333#install"
    ;;
esac

asset="333-${light}${arch}-${os}"
case "$os-$arch" in
  linux-x86_64) described="Linux on x86-64" ;;
  linux-aarch64) described="Linux on 64-bit ARM" ;;
  linux-armv6) described="Linux on 32-bit ARM, which the ARMv6 file covers" ;;
  macos-aarch64) described="macOS on Apple silicon" ;;
  macos-x86_64) described="macOS on Intel" ;;
esac
if [ -n "$light" ]; then form=Light; else form=Standard; fi

# Whichever of the two fetchers this system has. Both fail on an HTTP error rather than
# saving the error page under the name of the file.
if command -v curl >/dev/null 2>&1; then
  fetch() { curl --proto "$proto" --tlsv1.2 -fsSL -o "$2" "$1"; }
elif command -v wget >/dev/null 2>&1; then
  fetch() { wget -q -O "$2" "$1"; }
else
  fail "refused  Neither curl nor wget is on this system, so there is nothing to fetch the file with."
fi

# The system's own hashing tool, not one this script carries.
if command -v sha256sum >/dev/null 2>&1; then
  hash_of() { sha256sum "$1" | cut -d ' ' -f 1; }
elif command -v shasum >/dev/null 2>&1; then
  hash_of() { shasum -a 256 "$1" | cut -d ' ' -f 1; }
else
  fail "refused  Neither sha256sum nor shasum is on this system, so the file could not be checked," \
    "         and a file that has not been checked is not installed."
fi

manual() {
  printf '%s\n' "         To install it by hand instead, the commands are on https://the333.dev under" \
    "         'Start here', and the file itself is:" \
    "         $release_base/$asset" >&2
}

work=$(mktemp -d 2>/dev/null || mktemp -d -t 333)
trap 'rm -rf "$work"' EXIT
trap 'exit 130' INT TERM

say "machine  $described. Taking $form: $asset."

# The sums first, so that a release without them costs one small request rather than a
# twenty-megabyte download that is then thrown away.
if ! fetch "$release_base/SHA256SUMS" "$work/SHA256SUMS"; then
  printf '%s\n' "refused  Could not fetch SHA256SUMS from $release_base/SHA256SUMS." \
    "         Either this release carries none, or the network would not let it through. Without" \
    "         it there is nothing to check the file against, and an unchecked file is not installed." >&2
  manual
  exit 1
fi

expected=$(awk -v name="$asset" '$2 == name || $2 == "*" name { print tolower($1); exit }' "$work/SHA256SUMS")
if [ -z "$expected" ]; then
  printf '%s\n' "refused  SHA256SUMS in this release has no line for $asset," \
    "         so there is nothing to check it against, and it is not installed." >&2
  manual
  exit 1
fi

say "fetch    $release_base/$asset"
fetch "$release_base/$asset" "$work/$asset" ||
  fail "failed   Could not fetch $release_base/$asset. Nothing was installed."

actual=$(hash_of "$work/$asset" | tr 'A-F' 'a-f')
if [ "$actual" != "$expected" ]; then
  printf '%s\n' "refused  The file that arrived does not match SHA256SUMS, and it was not installed." \
    "         expected $expected" \
    "         arrived  $actual" \
    "         Something between here and the release changed it, or the release is broken." \
    "         It has been deleted." >&2
  exit 1
fi
say "checked  sha256 $actual, which is what SHA256SUMS says."

bin="$HOME/.local/bin"
dest="$bin/333"
mkdir -p "$bin" || fail "failed   Could not make $bin. Nothing was installed."
if [ -e "$dest" ]; then replacing=yes; else replacing=""; fi
# Copied in beside the old one and renamed over it, so a vigil already running from the old
# file keeps the file it started with, and nobody ever sees a half-written 333.
cp "$work/$asset" "$dest.new" || fail "failed   Could not write $dest.new. Nothing was installed."
chmod 755 "$dest.new" || fail "failed   Could not make $dest.new executable. Nothing was installed."
mv -f "$dest.new" "$dest" || fail "failed   Could not put the file at $dest. Nothing was installed."
if [ -n "$replacing" ]; then
  say "put      $dest, in place of the 333 that was there."
else
  say "put      $dest"
fi
say "         Nothing has been started, and nothing here will start on its own."

case ":${PATH:-}:" in
  *":$bin:"*) run=333 ;;
  *)
    run="$dest"
    say "path     $bin is not on your PATH, so until it is, the command is the whole path."
    say "         To put it there, add this line to your shell's startup file (~/.profile, or"
    say "         ~/.zshrc on a Mac) and open a new terminal:"
    say "         export PATH=\"\$HOME/.local/bin:\$PATH\""
    ;;
esac

say "next     $run join <invitation>   (one is on https://the333.dev/start)"
say "         $run start                 (runs it in the background)"
say "         $run                       (says what state it is in)"
