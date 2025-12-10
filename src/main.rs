use log::{debug, error, info, trace, warn};
use positive_tool_rs::pt;
use std;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let project_path: std::path::PathBuf = pt::find_project_root_path(env!("CARGO_PKG_NAME"))
        .ok()
        .unwrap();
    let test_tmp_file_path: std::path::PathBuf =
        project_path.clone().join("tmp_test_build_logger.log");
    pt::build_logger(test_tmp_file_path.clone(), None)
        .ok()
        .unwrap();
    trace!("{}的測試日志<追蹤>", PROJECT_NAME);
    debug!("{}的測試日志<除錯>", PROJECT_NAME);
    info!("{}的測試日志<資訊>", PROJECT_NAME);
    warn!("{}的測試日志<警告>", PROJECT_NAME);
    error!("{}的測試日志<錯誤>", PROJECT_NAME);
}
