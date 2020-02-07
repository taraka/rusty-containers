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

    let program_cstring = cstring(program);
    let arg_charptrs = cstring_array(args);


    unsafe {
        execv(program_cstring.as_ptr(), arg_charptrs.as_ptr());
    }
}

fn do_clone() -> bool {
    let pid = unsafe { fork() };
    return pid != 0;
}

fn cstring(input: &str) -> CString {
    CString::new(input.as_bytes()).unwrap()
}

fn cstring_array(input: &[&str]) -> Vec<*const i8> {
    let arr_cstrings = input.into_iter().map(|&arg| {
        CString::new(arg.as_bytes())
    }).collect::<Result<Vec<_>, _>>().unwrap();

    let mut arr_charptrs: Vec<_> = arr_cstrings.iter().map(|arg| {
        arg.as_ptr()
    }).collect();

    arr_charptrs.push(ptr::null());

    return arr_charptrs;
}