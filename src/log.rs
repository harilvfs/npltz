use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct Log;

impl Log {
    fn path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        base.join("npltz").join("npltz.log")
    }

    pub fn info(msg: &str) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(f, "[{ts}] INFO: {msg}");
        }
    }

    #[allow(dead_code)]
    pub fn error(msg: &str) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(f, "[{ts}] ERROR: {msg}");
        }
    }
}
