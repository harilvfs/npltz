use chrono::Local;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

const MAX_LOG_FILES: usize = 5;

/// Rate-limit flushes: a per-keypress fsync wastes battery and flash on mobile.
const FLUSH_INTERVAL: Duration = Duration::from_secs(1);

static SESSION_LOG: OnceLock<PathBuf> = OnceLock::new();
static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

struct Logger {
    writer:     Option<BufWriter<fs::File>>,
    last_flush: Instant,
}

pub struct Log;

impl Log {
    fn dir() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        base.join("npltz")
    }

    fn rotate() {
        let dir = Self::dir();
        let mut logs: Vec<PathBuf> = fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "log"))
            .collect();

        logs.sort();

        if logs.len() > MAX_LOG_FILES {
            for old in &logs[..logs.len() - MAX_LOG_FILES] {
                let _ = fs::remove_file(old);
            }
        }
    }

    pub fn init() {
        let dir = Self::dir();
        let _ = fs::create_dir_all(&dir);
        Self::rotate();

        let filename = Local::now().format("npltz-%Y%m%d-%H%M%S.log");
        let path = dir.join(filename.to_string());
        let _ = SESSION_LOG.set(path.clone());

        let writer = fs::OpenOptions::new().create(true).append(true).open(&path).ok();
        let _ = LOGGER.set(Mutex::new(Logger {
            writer:     writer.map(BufWriter::new),
            last_flush: Instant::now(),
        }));
    }

    fn write(level: &str, msg: &str) {
        let Some(logger_cell) = LOGGER.get() else { return };
        let Ok(mut guard) = logger_cell.lock() else {
            return;
        };
        let logger = &mut *guard;
        let Some(ref mut f) = logger.writer else { return };

        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(f, "[{ts}] [{level}] {msg}");

        if level == "ERROR" || logger.last_flush.elapsed() >= FLUSH_INTERVAL {
            let _ = f.flush();
            logger.last_flush = Instant::now();
        }
    }

    pub fn info(msg: &str) {
        Self::write("INFO", msg);
    }

    #[allow(dead_code)]
    pub fn error(msg: &str) {
        Self::write("ERROR", msg);
    }
}
