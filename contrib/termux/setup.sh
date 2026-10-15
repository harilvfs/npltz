#!/data/data/com.termux/files/usr/bin/sh
# Install the npltz Termux integration. This is a thin wrapper around
# `npltz setup --termux` so the file contents live in one place:
#
#   ~/.shortcuts/npltz            foreground shortcut (opens the TUI)
#   ~/.shortcuts/tasks/npltz-today background task (prints one line)
#   ~/.termux/extra-keys.json     key row with Tab, Esc and arrows
#
# Usage: sh contrib/termux/setup.sh [--boot] [--daily]
#   --boot   also install the boot notification script
#   --daily  also schedule a daily notification via termux-job-scheduler
set -eu

command -v npltz > /dev/null 2>&1 || {
    printf 'error: npltz not found in PATH (install it first, see the README)\n' >&2
    exit 1
}

exec npltz setup --termux "$@"
