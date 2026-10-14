#!/data/data/com.termux/files/usr/bin/sh
# Daily BS-date notification, scheduled by termux-job-scheduler.
#
#   sh contrib/termux/setup.sh --daily
#
# or manually:
#
#   termux-job-scheduler -s "$HOME/.shortcuts/tasks/npltz-notify-daily" \
#       --period-ms 86400000 --job-id 8310 --persisted --network-not-required
#
# Minimum period allowed by Android is 900000 ms (15 min); 86400000 = 24 h.
command -v termux-notification > /dev/null 2>&1 || exit 0
command -v npltz > /dev/null 2>&1 || exit 0

line=$(npltz show --oneline 2> /dev/null) || exit 0
[ -n "$line" ] || exit 0

termux-notification \
    --id npltz-today \
    --title "Today in Bikram Sambat" \
    --content "$line" \
    --priority default
