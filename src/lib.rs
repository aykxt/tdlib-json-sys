//! Bindings to the TDLib (Telegram Database library) JSON API.

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_double, c_int},
};

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
    fn td_execute(request: *const c_char) -> *const c_char;
}

#[derive(Debug, Clone)]
pub struct TDLib {
    client_id: i32,
}

impl TDLib {
    /// Creates a new instance of TDLib.
    pub fn new() -> Self {
        Self {
            client_id: unsafe { td_create_client_id() },
        }
    }

    /// Sends request to the TDLib client.
    ///
    /// May be called from any thread.
    pub fn send(&self, request: &str) {
        let cstring = CString::new(request).unwrap();
        unsafe { td_send(self.client_id, cstring.as_ptr()) }
    }

    /// Synchronously executes TDLib request.
    ///
    /// May be called from any thread. Only a few requests can be executed synchronously.
    pub fn execute(request: &str) -> Option<String> {
        let cstring = CString::new(request).unwrap();
        unsafe {
            td_execute(cstring.as_ptr())
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
        }
    }

    /// Receives incoming updates and request responses from the TDLib client.
    ///
    /// May be called from any thread, but shouldn't be called simultaneously
    /// from two different threads.
    pub fn receive(timeout: f64) -> Option<String> {
        unsafe {
            td_receive(timeout)
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
        }
    }
}

impl Default for TDLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::TDLib;

    #[test]
    fn test_execute() {
        //let client = TdClient::new();

        let result = TDLib::execute(
            r#"{"@type":"setLogVerbosityLevel","new_verbosity_level":1,"@extra":1.01234}"#,
        )
        .unwrap();

        assert_eq!(result, r#"{"@type":"ok","@extra":1.01234}"#);
    }
}
