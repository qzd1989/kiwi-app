#[macro_export]
macro_rules! project_info {
    ($($arg:tt)*) => {{
        let _ = $crate::app::get().try_with_project(|project| {
            project.log().info(format!($($arg)*))
        });
    }};
}

#[macro_export]
macro_rules! project_warn {
    ($($arg:tt)*) => {{
        let _ = $crate::app::get().try_with_project(|project| {
            project.log().warn(format!($($arg)*))
        });
    }};
}

#[macro_export]
macro_rules! project_error {
    ($($arg:tt)*) => {{
        let _ = $crate::app::get().try_with_project(|project| {
            project.log().error(format!($($arg)*))
        });
    }};
}

#[macro_export]
macro_rules! project_success {
    ($($arg:tt)*) => {{
        let _ = $crate::app::get().try_with_project(|project| {
            project.log().success(format!($($arg)*))
        });
    }};
}
