use libc::execv;
use std::ffi::CString;
use std::ptr;

fn main() {
    entry_point("/bin/ls", &["ls", "-la", "/"]);
    return()
}

fn entry_point(program: &str, args: &[&str]) -> () {

    let program_cstring = CString::new(program.as_bytes()).unwrap();

    let arg_cstrings = args.into_iter().map(|&arg| {
        CString::new(arg.as_bytes())
    }).collect::<Result<Vec<_>, _>>().unwrap();

    let mut arg_charptrs: Vec<_> = arg_cstrings.iter().map(|arg| {
        arg.as_ptr()
    }).collect();

    arg_charptrs.push(ptr::null());

    unsafe {
        execv(program_cstring.as_ptr(), arg_charptrs.as_ptr());
    }
}