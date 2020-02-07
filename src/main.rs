use libc::{execv, fork};
use std::ffi::CString;
use std::ptr;

fn main() {
    let is_child = do_clone();
    if is_child {
        entry_point("/usr/bin/hostname", &["hostname"]);
        return(); // Not needed as exec will holt execution but just here for clarity
    }

    println!("I'm here in the parent process");
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

fn do_clone() -> bool {
    let pid = unsafe { fork() };
    return pid != 0;
}