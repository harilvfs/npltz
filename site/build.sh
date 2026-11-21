#!/bin/sh
set -eu

ROOT=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
SITE="$ROOT/site"
OUT="$SITE/dist"

mkdir -p "$OUT/fonts"
cp "$SITE/index.html" "$SITE/styles.css" "$SITE/app.js" "$SITE/terminal.svg" "$OUT/"
cp "$SITE/fonts/inter-regular.ttf" "$SITE/fonts/inter-medium.ttf" "$SITE/fonts/inter-bold.ttf" "$SITE/fonts/OFL.txt" "$OUT/fonts/"
cp "$ROOT/calendar_data.json" "$OUT/calendar_data.json"
printf 'Static site ready in %s\n' "$OUT"
