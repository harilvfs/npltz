use chrono::Local;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const MAX_LOG_FILES: usize = 5;

static SESSION_LOG: OnceLock<PathBuf> = OnceLock::new();
static LOG_WRITER: OnceLock<Mutex<Option<BufWriter<fs::File>>>> = OnceLock::new();

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

        if let Ok(f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = LOG_WRITER.set(Mutex::new(Some(BufWriter::new(f))));
        }
    }

    fn write(level: &str, msg: &str) {
        let Some(writer) = LOG_WRITER.get() else { return };
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        if let Ok(mut guard) = writer.lock()
            && let Some(ref mut f) = *guard
        {
            let _ = writeln!(f, "[{ts}] [{level}] {msg}");
            let _ = f.flush();
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
