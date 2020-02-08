use nix::unistd::execv;
use std::ffi::{CString, CStr};
use std::ptr;

fn main() {
//    let is_child = do_clone();
//    if is_child {
        entry_point("/usr/bin/hostname", &["hostname"]);
        return(); // Not needed as exec will holt execution but just here for clarity
//    }
//
//    println!("I'm here in the parent process");
//    return()
}

fn entry_point(program: &str, args: &[&str]) -> () {

    let program_cstring = cstring(program);
    let arg_charptrs = cstring_array(args);


    unsafe {
        execv(program_cstring.as_c_str(), arg_charptrs.as_slice());
    }
}

//fn do_clone() -> bool {
//    let pid = unsafe { clone() };
//    return pid != 0;
//}

fn cstring(input: &str) -> CString {
    CString::new(input.as_bytes()).unwrap()
}

fn cstring_array(input: &[&str]) -> Vec<&'static CStr> {
    input.into_iter().map(|&arg| {
        CString::new(arg.as_bytes()).as_c_str()
    }).collect()
}