use nix::unistd::execv;
use std::ffi::{CString, CStr};

fn main() {
    let binary_path = CStr::from_bytes_with_nul(b"/bin/ls\0").unwrap();
    let binName = CStr::from_bytes_with_nul(b"ls\0").unwrap();
    let options = CStr::from_bytes_with_nul(b"-la\0").unwrap();
    let params = CStr::from_bytes_with_nul(b"/\0").unwrap();

    execv(binary_path, &[binName, options, params]);
    return()
}
