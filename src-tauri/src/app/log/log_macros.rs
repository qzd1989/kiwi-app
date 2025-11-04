#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::app::get().log().info(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::app::get().log().warn(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::app::get().log().error(format!($($arg)*));
    }};
}
