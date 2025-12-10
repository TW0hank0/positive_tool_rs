pub mod pt;

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
