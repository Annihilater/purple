use anyhow::Result;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LogConfig;

/// 日志守护者，确保日志文件写入
pub struct LogGuard {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

/// 初始化日志系统
///
/// 创建双输出日志系统：
/// - 终端输出：彩色格式，便于开发调试
/// - 文件输出：无色格式，便于日志分析和存储
pub fn init_logging(config: &LogConfig) -> Result<LogGuard> {
    let env_filter = parse_log_level(&config.level);

    // 创建文件输出器
    let file_appender = create_file_appender(&config.file_path)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 配置控制台日志层
    let console_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(config.with_thread_ids)
        .with_line_number(config.with_line_number)
        .with_file(config.with_file)
        .with_target(config.with_target);

    // 配置文件日志层
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false) // 文件中不使用颜色
        .with_thread_ids(config.with_thread_ids)
        .with_line_number(config.with_line_number)
        .with_file(config.with_file)
        .with_target(config.with_target);

    // 初始化订阅器
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(env_filter.into()))
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(LogGuard { _guard: guard })
}

/// 解析日志级别字符串
fn parse_log_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}

/// 创建文件日志输出器
fn create_file_appender(file_path: &str) -> Result<RollingFileAppender> {
    let log_file_path = std::path::Path::new(file_path);

    // 创建日志目录
    if let Some(parent) = log_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 获取日志目录和文件名
    let log_directory = log_file_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("logs"));

    let file_name = log_file_path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("app.log"))
        .to_string_lossy();

    // 解析文件名，获取基础名称和扩展名
    let (base_name, extension) = if let Some(pos) = file_name.rfind('.') {
        (&file_name[..pos], &file_name[pos..])
    } else {
        (file_name.as_ref(), "")
    };

    // 创建滚动文件输出器
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_directory,
        format!("{}{}", base_name, extension),
    );

    Ok(file_appender)
}
