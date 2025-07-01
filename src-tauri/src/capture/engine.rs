// done
use super::frame::Frame;
use crate::app::App;
use crate::types::Size;
use anyhow::{Result, anyhow};
use arc_swap::ArcSwapOption;
use capture::{Config, Engine as CaptureEngine, Frame as CaptureFrame};
use display_info::DisplayInfo;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

#[cfg(target_os = "macos")]
use objc2::msg_send;
#[cfg(target_os = "macos")]
use objc2_foundation::NSObject;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WINDOW_DISPLAY_AFFINITY};

pub struct Engine {
    engine: Arc<CaptureEngine>,
    frame: ArcSwapOption<Frame>,
}

impl Engine {
    pub fn start(&self) -> Result<()> {
        self.engine.start()
    }

    pub fn start_background(&self) -> Result<()> {
        self.engine.start_background()
    }

    pub fn stop(&self) {
        self.engine.stop();
    }

    pub fn is_running(&self) -> bool {
        self.engine.is_running()
    }

    pub fn grab(&self, time_out_millis: u64) -> Result<Frame> {
        let frame: Frame = if self.is_running() {
            self.get_frame()
                .ok_or_else(|| anyhow!(t!("Capturer is running, but no frame was captured.")))?
        } else {
            self.engine.grab(time_out_millis)?.into()
        };
        Ok(frame)
    }

    pub fn get_monitor_size(&self) -> Result<Size> {
        let infos = DisplayInfo::all()?;
        let Some(primary) = infos.into_iter().find(|info| info.is_primary) else {
            return Err(anyhow!(t!("No primary display found.")));
        };
        let size = Size {
            width: primary.width,
            height: primary.height,
        };
        Ok(size)
    }

    // 返回cloned,不影响原数据
    pub fn get_frame(&self) -> Option<Frame> {
        self.frame.load_full().as_ref().map(|arc| (**arc).clone())
    }

    // 返回arc,更快,只读
    pub fn get_frame_arc(&self) -> Option<Arc<Frame>> {
        self.frame.load_full()
    }

    pub fn clear_frame(&self) {
        self.frame.store(None);
    }
}

impl Engine {
    pub fn protect_windows(app_handle: &AppHandle, windows: &[String]) -> Result<()> {
        for window in windows {
            Self::protect_window(&app_handle, window)?;
        }
        Ok(())
    }

    pub fn unprotect_windows(app_handle: &AppHandle, windows: &[String]) -> Result<()> {
        for window in windows {
            Self::unprotect_window(&app_handle, window)?;
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn protect_window(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle.get_webview_window(label).ok_or_else(|| {
            anyhow!(t!(
                "Failed to retrieve webview with the specified label.",
                label = label
            ))
        })?;
        match window.hwnd() {
            Ok(hwnd) => unsafe {
                SetWindowDisplayAffinity(hwnd, WINDOW_DISPLAY_AFFINITY(17)).map_err(|e| {
                    anyhow!(t!(
                        "Failed to set window display affinity.",
                        error = e.to_string()
                    ))
                })
            },
            Err(e) => Err(anyhow!(t!(
                "Failed to find HWND by label.",
                label = label,
                error = e.to_string()
            ))),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn unprotect_window(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle.get_webview_window(label).ok_or_else(|| {
            anyhow!(t!(
                "Failed to retrieve webview with the specified label.",
                label = label
            ))
        })?;
        match window.hwnd() {
            Ok(hwnd) => unsafe {
                SetWindowDisplayAffinity(hwnd, WINDOW_DISPLAY_AFFINITY(0)).map_err(|e| {
                    anyhow!(t!(
                        "Failed to set window display affinity.",
                        error = e.to_string()
                    ))
                })
            },
            Err(e) => Err(anyhow!(t!(
                "Failed to find HWND by label.",
                label = label,
                error = e.to_string()
            ))),
        }
    }

    #[cfg(target_os = "macos")]
    fn protect_window(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle.get_webview_window(label).ok_or_else(|| {
            anyhow!(t!(
                "Failed to retrieve webview with the specified label.",
                label = label
            ))
        })?;
        let ns_window = window.ns_window()?;
        unsafe {
            let ns_object = &*(ns_window as *const NSObject);
            let _: () = msg_send![ns_object, setSharingType: 0u32];
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub fn unprotect_window(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle.get_webview_window(label).ok_or_else(|| {
            anyhow!(t!(
                "Failed to retrieve webview with the specified label.",
                label = label
            ))
        })?;
        let ns_window = window.ns_window()?;

        unsafe {
            let ns_object = &*(ns_window as *const NSObject);
            let _: () = msg_send![ns_object, setSharingType: 1u32];
        }

        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        let config = Config::new(capture::Format::RGBA);
        let on_frame_arrived = Box::new(|frame: CaptureFrame| {
            let new_frame = Frame::new(frame.width, frame.height, frame.buffer);
            App::with_capturer(|capturer| {
                capturer.frame.store(Some(Arc::new(new_frame)));
            });
        });
        let engine = CaptureEngine::new(config, on_frame_arrived);
        Self {
            engine,
            frame: ArcSwapOption::empty(),
        }
    }
}
