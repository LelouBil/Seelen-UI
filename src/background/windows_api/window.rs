use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use windows::Win32::Foundation::HWND;

use crate::error_handler::Result;

use super::{WindowEnumerator, WindowsApi};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window(HWND);

impl From<HWND> for Window {
    fn from(hwnd: HWND) -> Self {
        Self(hwnd)
    }
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.0 .0)
            .field("title", &self.title())
            .field("class", &self.class())
            .field("exe", &self.exe())
            .finish()
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Window({:x})", self.0 .0)
    }
}

pub const APP_FRAME_HOST_PATH: &str = "C:\\Windows\\System32\\ApplicationFrameHost.exe";
impl Window {
    pub fn hwnd(self) -> HWND {
        self.0
    }

    pub fn title(&self) -> String {
        WindowsApi::get_window_text(self.0)
    }

    pub fn class(&self) -> String {
        WindowsApi::get_class(self.0).unwrap_or_default()
    }

    pub fn exe(&self) -> Result<PathBuf> {
        WindowsApi::exe_path_v2(self.0)
    }

    pub fn app_display_name(&self) -> Result<String> {
        WindowsApi::get_window_display_name(self.0)
    }

    pub fn parent(&self) -> Option<Window> {
        let parent = WindowsApi::get_parent(self.0);
        if parent.0 != 0 {
            Some(Window(parent))
        } else {
            None
        }
    }

    pub fn children(&self) -> Result<Vec<Window>> {
        WindowEnumerator::new()
            .with_parent(self.0)
            .map(Window::from)
    }

    pub fn is_visible(&self) -> bool {
        WindowsApi::is_window_visible(self.0)
    }

    /// is the window an Application Frame Host
    pub fn is_frame(&self) -> Result<bool> {
        Ok(self.exe()? == PathBuf::from(APP_FRAME_HOST_PATH))
    }

    /// will fail if the window is not a frame
    pub fn get_frame_creator(&self) -> Result<Option<Window>> {
        if !self.is_frame()? {
            return Err("Window is not a frame".into());
        }
        for window in self.children()? {
            if !window.class().starts_with("ApplicationFrame") {
                return Ok(Some(window));
            }
        }
        Ok(None)
    }

    pub fn is_desktop(&self) -> bool {
        WindowsApi::get_desktop_window() == self.0 || self.class() == "Progman"
    }

    pub fn is_seelen_window(&self) -> bool {
        if let Ok(exe) = self.exe() {
            return exe.ends_with("seelen-ui.exe");
        }
        false
    }
}
