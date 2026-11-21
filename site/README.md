# npltz web calendar

The browser calendar is a static HTML, CSS and JavaScript site. It uses the same `calendar_data.json` file as the Rust app; it does not need a server API or a JavaScript framework. Inter is bundled locally under the SIL Open Font License.

## Run locally

From the repository root, run:

```sh
python3 serve.py
```

The script opens the site at `http://localhost:8000/site/`. Pass a starting port as the first argument, or use `--no-open` to keep it from opening a browser. If that port is busy, it tries the next available port. A web server is needed because browsers do not allow `fetch()` to load the calendar JSON from a `file://` URL.

## Build a self-contained deploy folder

From the repository root, run:

```sh
sh site/build.sh
```

This creates `site/dist/` with the page and a copy of the root calendar data. Deploy the contents of `site/dist/` to any static host. The generated folder is ignored by Git.

The site includes English and Nepali UI languages, updates its Nepal time clock every second, uses Nepal Standard Time (Asia/Kathmandu) for today, and supports BS 1975-2090. Its date conversion follows the epoch and year/month day-count rules in `src/calendar.rs`.
