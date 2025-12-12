//! ptrs 一個rust的工具

use colored::Color;
use log4rs;
use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::vec::Vec;

/// 找出專案資料夾
///
/// **NOTICE! this doctest will be failed with `cargo test` cause doctest doas not run under project path**
///
///  範例：
/// ```rust, no_run
/// //NOTICE! this test can NOT run as doctest.It will be failed when running as doctest with cargo cause doctest doas not run under project path
/// use ptrs::ptrs::*;
///
/// assert_eq!(
///            find_project_root_path(env!("CARGO_PKG_NAME"))
///                .ok()
///                .unwrap()
///                .file_name()
///                .unwrap()
///                .to_str()
///                .unwrap(),
///            "ptrs"
///        );
/// ```
pub fn find_project_root_path(project_name: &str) -> io::Result<PathBuf> {
    let exe_file_path: PathBuf;
    match env::current_exe() {
        Ok(p) => match p.canonicalize() {
            Ok(p_abs) => {
                exe_file_path = p_abs;
            }
            Err(e) => {
                return Err(e);
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
    //
    let mut project_path: PathBuf = exe_file_path;
    let mut project_path_count: u8 = 1;
    let mut project_path_log: Vec<PathBuf> = Vec::new();
    let max_dir_level: u8 = 10;
    let mut tmp_filename: &OsStr;
    //
    loop {
        match project_path.parent() {
            Some(p) => {
                project_path = p.to_path_buf();
            }
            None => {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
        }
        project_path_log.push(project_path.clone());
        match project_path.file_name() {
            Some(name) => tmp_filename = name,
            None => {
                return Err(io::Error::from(io::ErrorKind::InvalidFilename));
            }
        }
        if tmp_filename == project_name {
            break;
        } else {
            project_path_count += 1;
            if project_path_count >= max_dir_level {
                return Err(io::Error::from(io::ErrorKind::TimedOut));
            }
        }
    }
    return Ok(project_path);
}

/// 使用 `log4rs` 建立日志功能
///
/// **Args:**
///
/// ```rust, ignore
/// log_file_path: PathBuf
/// ```
///
/// 日志檔案的位子
///
/// **Return:**
///
/// ```rust, ignore
/// io::Result<()>
/// ```
pub fn build_logger(
    log_file_path: PathBuf,
    #[cfg(debug_assertions)] _release_log_file_level: Option<log::LevelFilter>,
    #[cfg(not(debug_assertions))] release_log_file_level: Option<log::LevelFilter>,
) -> io::Result<()> {
    let file_pattern: &str = "[{d(%Y-%m-%d %H:%M:%S)}] | {T} | {l} | [{f}:{L}::{M}] | {m}{n}";

    // ----------------------------------------------------
    // 建立 FileHandler (檔案輸出)
    // ----------------------------------------------------
    /*let file_appender: log4rs::append::file::FileAppender =
    log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            file_pattern,
        )))
        .append(true)
        .build(log_file_path)
        .expect("無法建立檔案 appender");
    */
    let file_appender: log4rs::append::file::FileAppender;
    match log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            file_pattern,
        )))
        .append(true)
        .build(log_file_path)
    {
        Ok(i) => {
            file_appender = i;
        }
        Err(e) => {
            return Err(e);
        }
    }
    //建立 console appender
    let console_pattern: String = format!(
        "\x1b[{}m[{{d(%Y-%m-%d %H:%M:%S)}}]\x1b[{}m | {{l}} | \x1b[{}m[\x1b[{}m{{f}}\x1b[{}m:{{L}}::{{M}}\x1b[{}m]\x1b[{}m | {{m}}{{n}}",
        Color::to_fg_str(&Color::Green),
        Color::to_fg_str(&Color::White),
        Color::to_fg_str(&Color::Cyan),
        Color::to_fg_str(&Color::Magenta),
        Color::to_fg_str(&Color::Yellow),
        Color::to_fg_str(&Color::Cyan),
        Color::to_fg_str(&Color::White),
    );
    let console_appender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            &console_pattern,
        )))
        .build();
    // 建立總設定 Config
    #[cfg(debug_assertions)]
    let prepare_config_file_filter = {
        Box::new(log4rs::filter::threshold::ThresholdFilter::new(
            log::LevelFilter::Trace,
        ))
    };
    #[cfg(not(debug_assertions))]
    let prepare_config_file_filter = {
        match release_log_file_level {
            Some(i) => Box::new(log4rs::filter::threshold::ThresholdFilter::new(i)),
            _ => Box::new(log4rs::filter::threshold::ThresholdFilter::new(
                log::LevelFilter::Info,
            )),
        }
    };
    //
    let config = log4rs::config::Config::builder()
        // 註冊檔案 appender
        .appender(
            log4rs::config::Appender::builder()
                .filter(prepare_config_file_filter)
                .build("file_logger", Box::new(file_appender)),
        )
        // 註冊終端 appender
        .appender(
            log4rs::config::Appender::builder()
                .filter(Box::new(log4rs::filter::threshold::ThresholdFilter::new(
                    log::LevelFilter::Warn,
                )))
                .build("console_logger", Box::new(console_appender)),
        )
        // 5. 設定 Root Logger
        .build(
            log4rs::config::Root::builder()
                .appender("file_logger")
                .appender("console_logger")
                .build(log::LevelFilter::Trace),
        )
        .expect("無法建立日誌配置");
    // 初始化日誌系統
    match log4rs::init_config(config) {
        Ok(_handle) => {
            return Ok(());
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pt::*;
    use log::{debug, error, info, trace, warn};
    use std::fs;

    #[test]
    fn test_find_project_root_path() {
        assert_eq!(
            find_project_root_path(env!("CARGO_PKG_NAME"))
                .ok()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            "ptrs"
        );
    }

    #[test]
    fn test_build_logger() {
        let test_tmp_file_path = find_project_root_path(env!("CARGO_PKG_NAME"))
            .ok()
            .unwrap()
            .join("tmp_test_build_logger.log");
        build_logger(test_tmp_file_path.clone(), None).ok().unwrap();
        trace!("測試日志<追蹤>");
        debug!("測試日志<除錯>");
        info!("測試日志<資訊>");
        warn!("測試日志<警告>");
        error!("測試日志<錯誤>");
        assert!(fs::exists(test_tmp_file_path).ok().unwrap());
    }
}
