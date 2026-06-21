use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

const MAX_LOG_FILES: usize = 5;

static SESSION_LOG: OnceLock<PathBuf> = OnceLock::new();

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
        let _ = SESSION_LOG.set(path);
    }

    fn write(level: &str, msg: &str) {
        let Some(path) = SESSION_LOG.get() else { return };
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "[{ts}] [{level}] {msg}");
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
