use nix::unistd::{execv, gethostname, sethostname, getpid, getppid, chroot, mkdir, chdir, Pid};
use nix::sched::{CloneFlags, clone};
use nix::sys::stat;
use nix::mount::{mount, MsFlags};
use std::ffi::{CString, CStr};
use std::str::from_utf8;
use std::path::Path;
use std::{time, thread};
use nix::sys::wait::waitpid;

const BASE_DIR: &str = "/home/tom/Code/rusty-containers/containers";
static name: &str = "container1";

fn main() {
    create_container();
}

fn create_container() {
    let container_base = String::from(BASE_DIR) + "/" + name;
    mkdir::<str>(&container_base[..], stat::Mode::S_IRWXU | stat::Mode::S_IRGRP | stat::Mode::S_IXGRP);

    let child = do_clone();
    println!("Child PID: {}", child);
    waitpid(child, None);



//    sleep(1);

//    let ref mut buf: [u8; 256] = [0; 256];
//    gethostname(buf).expect(-"Getting hostname failed");
//
//    println!("I'm here in the parent process on : {}", from_utf8(buf).expect("Get hostname from string failed"));
    return()
}

fn child() -> isize {

    //thread::sleep(time::Duration::from_secs(180));
    let container_base = String::from(BASE_DIR) + "/" + name;

    chroot(&container_base[..]);
    chdir("/");

    mount::<str, str,  str, str>(Some("proc"), "proc", Some("proc"), MsFlags::MS_REC ^ MsFlags::MS_REC, None).expect("Mount proc failed");

    sethostname(name).expect("Setting hostname failed");

    println!("Child PID: {}", getpid());
    println!("Parent PID: {}", getppid());

    entry_point("/bin/ls", &["ls", "-la", "/proc"]);

    return 0;
}

fn do_clone() -> Pid {
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

    clone(cbk, stack, flag_bits , None).expect("Clone Failed")
}

fn entry_point(program: &str, args: &[&str]) -> () {

    let program_cstring = cstring(program);
    let arg_charptrs = cstring_array(args);


    execv(program_cstring.as_c_str(), arg_charptrs.as_slice()).expect("Exec failed");
}

fn cstring(input: &str) -> CString {
    CString::new(input.as_bytes()).unwrap()
}

fn cstring_array<'a>(input: &[&str]) -> Vec<&'a CStr> {
    input.into_iter().map(|&arg| {
        Box::leak(Box::new(CString::new(arg).unwrap())).as_c_str()
    }).collect()
}