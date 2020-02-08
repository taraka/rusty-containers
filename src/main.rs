use nix::unistd::{execv, gethostname, sethostname, sleep};
use nix::sched::{CloneFlags, clone};
use std::ffi::{CString, CStr};
use std::str::from_utf8;


fn main() {
    create_container();
}

fn create_container() {
    do_clone();

    sleep(1);

    let ref mut buf: [u8; 256] = [0; 256];
    gethostname(buf).expect("Getting hostname failed");

    println!("I'm here in the parent process on : {}", from_utf8(buf).expect("Get hostname from string failed"));
    return()
}

fn child() -> isize {

    sethostname("newhostname").expect("Setting hostname failed");
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

    let flag_bits: CloneFlags =
        CloneFlags::CLONE_NEWCGROUP |
        CloneFlags::CLONE_NEWUTS |
        CloneFlags::CLONE_NEWIPC |
        CloneFlags::CLONE_NEWUSER |
        CloneFlags::CLONE_NEWPID |
        CloneFlags::CLONE_NEWNET |
        CloneFlags::CLONE_NEWNS;

    let p = clone(cbk, stack, flag_bits , None);
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