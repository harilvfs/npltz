#!/bin/sh
set -e

REPO="harilvfs/npltz"
BINARY="npltz"

if [ -n "$TERMUX_VERSION" ] || [ -d "/data/data/com.termux" ]; then
    IS_ANDROID=true
else
    IS_ANDROID=false
fi

detect_target() {
    ARCH=$(uname -m)
    OS=$(uname -s)
    if [ "$OS" != "Linux" ]; then
        printf "unsupported OS: %s\n" "$OS" >&2
        exit 1
    fi
    if [ "$IS_ANDROID" = "true" ]; then
        case "$ARCH" in
            aarch64 | arm64) echo "aarch64-linux-android" ;;
            armv7* | armv8l | arm) echo "armv7-linux-androideabi" ;;
            *)
                printf "unsupported architecture: %s\n" "$ARCH" >&2
                exit 1
                ;;
        esac
    else
        case "$ARCH" in
            x86_64 | amd64) echo "x86_64-unknown-linux-musl" ;;
            aarch64 | arm64) echo "aarch64-unknown-linux-musl" ;;
            *)
                printf "unsupported architecture: %s\n" "$ARCH" >&2
                exit 1
                ;;
        esac
    fi
}

get_latest_version() {
    curl -fsL "https://api.github.com/repos/$REPO/releases/latest" 2> /dev/null |
        grep '"tag_name"' | head -1 | cut -d'"' -f4
}

VERSION="${NPLTZ_VERSION:-latest}"
TARGET=$(detect_target)

[ "$VERSION" = "latest" ] && VERSION=$(get_latest_version)

if [ -z "$VERSION" ]; then
    printf "error: could not determine release version\n" >&2
    exit 1
fi

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

ARCHIVE="${BINARY}-${TARGET}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"

printf "[ %s ] platform: %s\n" "$BINARY" "$TARGET"
printf "[ %s ] version:  %s\n" "$BINARY" "$VERSION"

printf "[ fetching ] archive... "
if ! curl -fsSL "$URL" -o "$TMPDIR/$ARCHIVE"; then
    printf "failed\n"
    exit 1
fi
printf "done\n"

printf "[ verify   ] checksum... "
if ! curl -fsSL "$URL.sha256" -o "$TMPDIR/$ARCHIVE.sha256"; then
    printf "failed\n"
    exit 1
fi
printf "done\n"

cd "$TMPDIR"
if ! sha256sum -c "$ARCHIVE.sha256" > /dev/null 2>&1; then
    printf "failed\n"
    exit 1
fi
printf "[ checksum ] ok\n"

printf "[ extract  ] archive... "
tar xzf "$ARCHIVE"
RELEASE_DIR="${BINARY}-${TARGET}"
cd "$RELEASE_DIR"
printf "done\n"

printf "[ install  ] binary... "
if [ "$IS_ANDROID" = "true" ]; then
    if ! install -Dm755 "$BINARY" "$PREFIX/bin/$BINARY"; then
        printf "failed\n"
        exit 1
    fi
else
    if ! sudo install -Dm755 "$BINARY" "/usr/local/bin/$BINARY"; then
        printf "failed\n"
        exit 1
    fi
fi
printf "done\n"

if [ -d "completions" ]; then
    printf "[ completions ] installing... "
    if [ "$IS_ANDROID" = "true" ]; then
        mkdir -p "$PREFIX/share/bash-completion/completions" \
            "$PREFIX/share/zsh/site-functions" \
            "$PREFIX/share/fish/vendor_completions.d"
        [ -f "completions/npltz.bash" ] && cp "completions/npltz.bash" "$PREFIX/share/bash-completion/completions/npltz"
        [ -f "completions/npltz.zsh" ] && cp "completions/npltz.zsh" "$PREFIX/share/zsh/site-functions/_npltz"
        [ -f "completions/npltz.fish" ] && cp "completions/npltz.fish" "$PREFIX/share/fish/vendor_completions.d/npltz.fish"
    else
        sudo mkdir -p /usr/share/bash-completion/completions \
            /usr/share/zsh/site-functions \
            /usr/share/fish/vendor_completions.d
        [ -f "completions/npltz.bash" ] && sudo cp "completions/npltz.bash" /usr/share/bash-completion/completions/npltz
        [ -f "completions/npltz.zsh" ] && sudo cp "completions/npltz.zsh" /usr/share/zsh/site-functions/_npltz
        [ -f "completions/npltz.fish" ] && sudo cp "completions/npltz.fish" /usr/share/fish/vendor_completions.d/npltz.fish
    fi
    printf "done\n"
fi

if [ -f "man/npltz.1" ]; then
    printf "[ manpage  ] installing... "
    if [ "$IS_ANDROID" = "true" ]; then
        if ! install -Dm644 "man/npltz.1" "$PREFIX/share/man/man1/npltz.1"; then
            printf "failed\n"
            exit 1
        fi
    else
        if ! sudo install -Dm644 "man/npltz.1" "/usr/share/man/man1/npltz.1"; then
            printf "failed\n"
            exit 1
        fi
    fi
    printf "done\n"
    mandb -q 2> /dev/null || true
fi

if [ "$IS_ANDROID" = "false" ] && [ -f "npltz.desktop" ]; then
    printf "[ desktop  ] installing... "
    if ! sudo install -Dm644 "npltz.desktop" "/usr/share/applications/npltz.desktop"; then
        printf "failed\n"
        exit 1
    fi
    printf "done\n"
fi

printf "\n[ done ] npltz %s installed successfully\n" "$VERSION"
printf "[ done ] run npltz to get started\n"
