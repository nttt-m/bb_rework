use flexi_logger::{
    Cleanup, Criterion, FileSpec, FlexiLoggerError, Logger, LoggerHandle, Naming, WriteMode,
};

pub fn init_logger() -> Result<LoggerHandle, FlexiLoggerError> {
    Logger::try_with_env_or_str("trace")?
        .log_to_file(FileSpec::default().directory("/var/log/bb-hrms"))
        .write_mode(WriteMode::Async)
        .rotate(
            Criterion::Size(10000000),
            Naming::Timestamps,
            Cleanup::KeepLogAndCompressedFiles(5, 20),
        )
        .cleanup_in_background_thread(true)
        .duplicate_to_stdout(flexi_logger::Duplicate::Trace)
        .start()
}