use nix::unistd::execv;
use nix::sched;
use std::ffi::{CString, CStr};

fn main() {

    do_clone();

    println!("I'm here in the parent process");
    return()
}

fn child() -> isize {
    entry_point("/bin/hostname", &["hostname"]);
    return 0;
}

fn entry_point(program: &str, args: &[&str]) -> () {

    let program_cstring = cstring(program);
    let arg_charptrs = cstring_array(args);


    execv(program_cstring.as_c_str(), arg_charptrs.as_slice()).expect("Exec failed");
}

fn do_clone() {
    const STACK_SIZE: usize = 1024 * 1024;
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let cbk = Box::new(|| child());
    let p = sched::clone(cbk, stack, sched::CloneFlags::CLONE_NEWUTS, None);
    let _p = match p {
        Ok(p) => p,
        Err(err) => panic!(err),
    };
}



fn cstring(input: &str) -> CString {
    CString::new(input.as_bytes()).unwrap()
}

fn cstring_array<'a>(input: &[&str]) -> Vec<&'a CStr> {
    input.into_iter().map(|&arg| {
        Box::leak(Box::new(CString::new(arg).unwrap())).as_c_str()
    }).collect()
}