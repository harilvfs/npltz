#!/data/data/com.termux/files/usr/bin/sh
# Post a notification with today's BS date when the device boots.
# Installed to ~/.termux/boot/ by contrib/termux/setup.sh --boot.
command -v termux-notification > /dev/null 2>&1 || exit 0
command -v npltz > /dev/null 2>&1 || exit 0

# Wait for boot to settle before pinging the notification service.
sleep 20

line=$(npltz show --oneline 2> /dev/null) || exit 0
[ -n "$line" ] || exit 0

termux-notification \
    --id npltz-today \
    --title "Today in Bikram Sambat" \
    --content "$line" \
    --priority low
