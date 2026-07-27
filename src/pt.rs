//! positive_tool_rs 一個開發工具

use std::{io, path::PathBuf};

#[cfg(feature = "log")]
use colored::Color;
#[cfg(feature = "log")]
use log4rs;

#[cfg(feature = "tracing")]
use tracing_appender;
#[cfg(feature = "tracing")]
use tracing_subscriber::{self, Layer, layer::SubscriberExt, util::SubscriberInitExt};

/// 初始化 tracing 日誌
///
/// ```rust, no_run
/// use std::path::PathBuf;
/// use positive_tool_rs::pt::init_tracing;
///
/// let _guard = init_tracing(PathBuf::from("logs"), None);
/// ```
#[cfg(feature = "tracing")]
pub fn init_tracing(
    log_dir: PathBuf,
    file_name_prefix: Option<String>,
) -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender: tracing_appender::rolling::RollingFileAppender;
    match tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix(file_name_prefix.unwrap_or(String::from("log")))
        .filename_suffix(String::from("log"))
        .max_log_files(100)
        .build(log_dir)
    {
        Ok(item) => {
            file_appender = item;
        }
        Err(err) => {
            panic!("file appender build failed, err: {:?}", err);
        }
    }
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(io::stdout)
        .with_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or(tracing_subscriber::EnvFilter::new("info")),
        );
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_filter(tracing_subscriber::EnvFilter::new("debug"));
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
    guard
}

/// 使用 `log4rs` 初始化日誌
#[cfg(feature = "log")]
pub fn build_logger(
    log_file_path: PathBuf,
    #[cfg(debug_assertions)] _release_log_file_level: Option<log::LevelFilter>,
    #[cfg(not(debug_assertions))] release_log_file_level: Option<log::LevelFilter>,
) -> io::Result<()> {
    let file_pattern: &str = "[{d(%Y-%m-%d %H:%M:%S)}] | {T} | {l} | [{f}:{L}::{M}] | {m}{n}";
    let config_builder = log4rs::config::Config::builder();
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
    // 註冊檔案 appender
    let config_builder = config_builder.appender(
        log4rs::config::Appender::builder()
            .filter(prepare_config_file_filter)
            .build("file_logger", Box::new(file_appender)),
    );
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

    // 註冊終端 appender
    let config_builder = config_builder.appender(
        log4rs::config::Appender::builder()
            .filter(Box::new(log4rs::filter::threshold::ThresholdFilter::new(
                log::LevelFilter::Warn,
            )))
            .build("console_logger", Box::new(console_appender)),
    );
    // 5. 設定 Root Logger
    let config = config_builder
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
    use std::{fs, path::PathBuf};

    #[cfg(feature = "log")]
    use crate::pt::build_logger;
    #[cfg(feature = "tracing")]
    use crate::pt::init_tracing;
    #[cfg(feature = "log")]
    #[test]
    fn test_build_logger() {
        use log::{debug, error, info, trace, warn};
        build_logger(
            PathBuf::from("target").join("test").join("tmp_log.log"),
            None,
        )
        .ok();
        trace!("測試日志<追蹤>");
        debug!("測試日志<除錯>");
        info!("測試日志<資訊>");
        warn!("測試日志<警告>");
        error!("測試日志<錯誤>");
        assert!(
            fs::exists(PathBuf::from("target").join("test").join("tmp_log.log"))
                .ok()
                .unwrap()
        );
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn test_init_tracing() {
        use tracing::{debug, error, info, trace, warn};
        if !fs::exists(PathBuf::from("target").join("test").join("init_tracing")).unwrap_or(false) {
            fs::create_dir_all(PathBuf::from("target").join("test").join("init_tracing")).ok();
        }
        let _guard = init_tracing(
            PathBuf::from("target").join("test").join("init_tracing"),
            None,
        );
        trace!("測試日志<追蹤>");
        debug!("測試日志<除錯>");
        info!("測試日志<資訊>");
        warn!("測試日志<警告>");
        error!("測試日志<錯誤>");
    }
}
