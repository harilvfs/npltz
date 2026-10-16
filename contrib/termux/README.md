# npltz on Termux

Small helpers so the calendar behaves well on a phone: a session-menu
shortcut, a one-line "glance" task, boot/daily notifications, and an
`extra-keys` layout with the keys npltz actually uses.

## Install

Everything below is written by the binary itself, so either way works:

```sh
npltz setup --termux               # shortcuts + extra keys
npltz setup --termux --boot        # + notification on boot
npltz setup --termux --daily       # + daily notification job
npltz setup --termux --dry-run     # preview without writing anything
```

or, equivalently, through the wrapper script in this folder:

```sh
sh contrib/termux/setup.sh            # shortcuts only
sh contrib/termux/setup.sh --boot     # + notification on boot
sh contrib/termux/setup.sh --daily    # + daily 08:00 notification job
```

`--boot` and `--daily` need the `termux-api` package and the
[Termux:API](https://f-droid.org/en/packages/com.termux.api/) app:

```sh
pkg install termux-api
```

## What gets installed

| Path                                              | Purpose                                          |
| ------------------------------------------------- | ------------------------------------------------ |
| `~/.shortcuts/npltz`                              | Opens the TUI from the Termux session menu.      |
| `~/.shortcuts/tasks/npltz-today`                  | Prints `npltz show --oneline` (Termux:Widget friendly). |
| `~/.termux/boot/10-npltz-notify.sh`               | Posts today's BS date as a notification at boot. |
| `~/.shortcuts/tasks/npltz-notify-daily`           | Same notification, run daily by `termux-job-scheduler`. |

Both shortcut directories must be mode `0700` and the scripts executable —
`setup.sh` takes care of that. After installing, restart the Termux session
(long-press volume-down + q, then *New session*) so the shortcut menu picks
them up.

## Extra keys

The default Termux key row has no `Tab`, `Esc` or arrows, which makes the
day cursor awkward. `npltz setup --termux` writes one; to copy it manually
from this repo:

```sh
mkdir -p ~/.termux
cp contrib/termux/extra-keys.json ~/.termux/extra-keys.json
termux-reload-settings
```

An existing `~/.termux/extra-keys.json` is never overwritten.

```
Esc   Tab   h    j    k    l    q
CTRL  ←    ↓    ↑    →    a    ?
```

- `h` / `l` - previous / next month (`Tab` + arrows move by day instead)
- `Tab` - start/stop the day cursor, arrows then move ±1 / ±7 days
- `a` - show/hide the AD sub-row under each BS week
- `?` - help, `q` - quit (in cursor mode, `Esc` leaves the cursor first)

## Notes

- The TUI adapts to the terminal size: portrait phones get a borderless
  layout, landscape gets a status line instead of the side panel. No need to
  resize anything manually, and rotation is handled live.
- `npltz show --oneline` is the cheap path for widgets and notifications: it
  does one date lookup, no TUI, no network.
- The default build is fully offline (calendar data is embedded), so nothing
  here needs connectivity.
- If your prompt or widget honors `NO_COLOR`, run `npltz --no-color` (or set
  the env var) for a plain, modifier-only rendering that survives every
  terminal color scheme.

## Uninstall

```sh
rm -f ~/.shortcuts/npltz \
      ~/.shortcuts/tasks/npltz-today \
      ~/.shortcuts/tasks/npltz-notify-daily \
      ~/.termux/boot/10-npltz-notify.sh
termux-job-scheduler --job-id 8310 --cancel
```
