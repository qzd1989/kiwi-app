#[macro_export]
macro_rules! app_info {
    ($($arg:tt)*) => {
        crate::commands::frontend::utils::log::Log::info(format!($($arg)*)).to_main()
    };
}

#[macro_export]
macro_rules! app_warn {
    ($($arg:tt)*) => {
        crate::commands::frontend::utils::log::Log::warn(format!($($arg)*)).to_main()
    };
}

#[macro_export]
macro_rules! app_error {
    ($($arg:tt)*) => {
        crate::commands::frontend::utils::log::Log::error(format!($($arg)*)).to_main()
    };
}
