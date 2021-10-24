use alloc::string::String;
use nx::fs;

pub const LOG_FILE: &str = "sdmc:/ldn_mitm/ldn_mitm.log";

pub const fn is_logging_enabled() -> bool {
    true
}

pub fn initialize() {
    let _ = fs::delete_file(String::from(LOG_FILE));
}

fn log_impl(line: String) {
    if is_logging_enabled() {
        if let Ok(mut log_file) = fs::open_file(
            String::from(LOG_FILE),
            fs::FileOpenOption::Create()
                | fs::FileOpenOption::Write()
                | fs::FileOpenOption::Append(),
        ) {
            let _ = log_file.write(line.as_ptr(), line.len());
        };
    }
}

pub fn log_line(text: impl AsRef<str>) {
    log_impl(format!("{}\n", text.as_ref()));
}
