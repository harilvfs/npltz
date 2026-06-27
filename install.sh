#!/bin/sh
set -e

REPO="harilvfs/npltz"
BINARY="npltz"

if [ -n "$TERMUX_VERSION" ] || [ -d "/data/data/com.termux" ]; then
    IS_ANDROID=true
else
    IS_ANDROID=false
fi

if [ "$IS_ANDROID" = "true" ] && [ -z "$PREFIX" ]; then
    PREFIX="/data/data/com.termux/files/usr"
fi

detect_target() {
    ARCH=$(uname -m)
    OS=$(uname -s)
    if [ "$IS_ANDROID" = "true" ]; then
        case "$ARCH" in
            aarch64 | arm64)
                echo "aarch64-linux-android"
                ;;
            armv7* | armv8l | arm)
                echo "armv7-linux-androideabi"
                ;;
            *)
                echo "unsupported architecture: $ARCH" >&2
                exit 1
                ;;
        esac
        return
    fi
    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64 | amd64)
                    echo "x86_64-unknown-linux-musl"
                    ;;
                aarch64 | arm64)
                    echo "aarch64-unknown-linux-musl"
                    ;;
                *)
                    echo "unsupported architecture: $ARCH" >&2
                    exit 1
                    ;;
        esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64)
                    echo "x86_64-apple-darwin"
                    ;;
                arm64)
                    echo "aarch64-apple-darwin"
                    ;;
                *)
                    echo "unsupported architecture: $ARCH" >&2
                    exit 1
                    ;;
        esac
            ;;
        *)
            echo "unsupported OS: $OS" >&2
            exit 1
            ;;
    esac
}

get_latest_version() {
    _tag=$(curl -fsL "https://api.github.com/repos/$REPO/releases/latest" 2> /dev/null |
          grep '"tag_name"' | head -1 | cut -d'"' -f4)
    case "$_tag" in
        v[0-9]*) echo "$_tag" ;;
        *) echo "" ;;
    esac
}

VERSION="${NPLTZ_VERSION:-latest}"
TARGET=$(detect_target)

if [ "$VERSION" = "latest" ]; then
    VERSION=$(get_latest_version)
fi

if [ -z "$VERSION" ]; then
    echo "error: could not determine release version" >&2
    exit 1
fi

echo "Installing $BINARY $VERSION ($TARGET)"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

ASSET="$BINARY-$TARGET"
curl -fsSL "https://github.com/$REPO/releases/download/$VERSION/$ASSET" -o "$TMPDIR/$BINARY"
curl -fsSL "https://github.com/$REPO/releases/download/$VERSION/$ASSET.sha256" -o "$TMPDIR/$BINARY.sha256"

cd "$TMPDIR"
sha256sum -c "$BINARY.sha256"
chmod 755 "$BINARY"

if [ "$IS_ANDROID" = "true" ]; then
    install -Dm755 "$BINARY" "$PREFIX/bin/$BINARY"
elif [ "$(uname -s)" = "Darwin" ]; then
    mkdir -p /usr/local/bin
    install -m755 "$BINARY" "/usr/local/bin/$BINARY"
else
    sudo mkdir -p /usr/local/bin
    sudo install -m755 "$BINARY" "/usr/local/bin/$BINARY"
fi

echo "$BINARY installed successfully"
echo "Run '$BINARY setup' to install completions and man page"
