use glib::error;
use gtk::glib;
use std::ffi::CStr;

pub const G_LOG_DOMAIN: &str = "Terminal";
pub const FALLBACK_SHELL: &str = "/bin/sh";

pub fn get_shell() -> String {
    unsafe {
        let passwd = libc::getpwuid(libc::getuid());
        let shell = CStr::from_ptr((*passwd).pw_shell).to_str();
        if let Ok(shell) = shell
            && !shell.is_empty()
        {
            return shell.to_string();
        }
    }

    error!(
        "Login shell could not be detected. Falling back to {:?}.",
        FALLBACK_SHELL
    );
    FALLBACK_SHELL.to_string()
}
