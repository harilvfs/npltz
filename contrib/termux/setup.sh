#!/data/data/com.termux/files/usr/bin/sh
# Install the npltz Termux integration:
#
#   ~/.shortcuts/npltz            foreground shortcut (opens the TUI)
#   ~/.shortcuts/tasks/npltz-today background task (prints one line)
#   ~/.termux/boot/10-npltz-notify.sh  notification with today's date on boot
#
# Usage: sh contrib/termux/setup.sh [--boot] [--daily]
#   --boot   also install the boot notification script
#   --daily  also schedule a daily 08:00 notification via termux-job-scheduler
set -eu

HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)

die() {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

if ! command -v npltz > /dev/null 2>&1; then
    die "npltz not found in PATH (install it first, see the README)"
fi

mkdir -p "$HOME/.shortcuts/tasks"
chmod 700 "$HOME/.shortcuts" "$HOME/.shortcuts/tasks"

install -m 755 "$HERE/shortcuts/npltz" "$HOME/.shortcuts/npltz"
install -m 755 "$HERE/shortcuts/tasks/npltz-today" "$HOME/.shortcuts/tasks/npltz-today"

printf 'installed: ~/.shortcuts/npltz and ~/.shortcuts/tasks/npltz-today\n'

case "${1:-}" in
--boot)
    if ! command -v termux-notification > /dev/null 2>&1; then
        die "termux-api package is required (pkg install termux-api)"
    fi
    mkdir -p "$HOME/.termux/boot"
    install -m 755 "$HERE/boot/10-npltz-notify.sh" "$HOME/.termux/boot/10-npltz-notify.sh"
    printf 'installed: ~/.termux/boot/10-npltz-notify.sh\n'
    ;;
--daily)
    if ! command -v termux-job-scheduler > /dev/null 2>&1; then
        die "termux-api package is required (pkg install termux-api)"
    fi
    install -m 755 "$HERE/notify-daily.sh" "$HOME/.shortcuts/tasks/npltz-notify-daily"
    termux-job-scheduler -s "$HOME/.shortcuts/tasks/npltz-notify-daily" \
        --period-ms 86400000 --job-id 8310 --persisted --network-not-required
    printf 'scheduled: daily notification job (job-id 8310, every 24h)\n'
    ;;
esac

printf 'done. open the TUI from the Termux session menu ("npltz" shortcut).\n'
