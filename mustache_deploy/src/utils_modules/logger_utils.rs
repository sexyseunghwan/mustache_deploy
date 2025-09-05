use crate::common::*;

/// 전역 로거를 설정하는 함수
/// 
/// # 기능
/// - 일별 로그 로테이션을 지원하는 파일 기반 로거 설정
/// - 최대 10개의 로그 파일 보관
/// - 커스텀 포맷으로 로그 출력 (날짜, 레벨, 스레드 명, 메시지)
/// 
/// # Panic
/// - 로거 초기화에 실패하면 panic 발생
pub fn set_global_logger() {
    let log_directory = LOG_DIRECTORY; // Directory to store log files
    let file_prefix = ""; // Prefixes for log files

    // Logger setting
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory(log_directory)
                .discriminant(file_prefix),
        )
        .rotate(
            Criterion::Age(Age::Day),  // daily rotation
            Naming::Timestamps,        // Use timestamps for file names
            Cleanup::KeepLogFiles(LOG_FILES_TO_KEEP), // Maintain log files
        )
        .format_for_files(custom_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));
}

/// 커스텀 로그 포맷 함수
/// 
/// # Arguments
/// * `w` - 로그를 출력할 Writer
/// * `now` - 현재 시간 정보
/// * `record` - 로그 레코드 (레벨, 메시지 등 포함)
/// 
/// # Returns
/// * `Ok(())` - 로그 포맷팅 성공
/// * `Err(std::io::Error)` - 로그 쓰기 실패
/// 
/// # Format
/// `[YYYY-MM-DD HH:MM:SS] [LEVEL] T[thread_name] message`
fn custom_format(
    w: &mut dyn Write,
    now: &mut flexi_logger::DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] [{}] T[{}] {}",
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        std::thread::current().name().unwrap_or("unknown"),
        &record.args()
    )
}
