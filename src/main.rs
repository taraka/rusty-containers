use nix::unistd::execv;
use nix::unistd::{fork, ForkResult};
use std::ffi::{CString, CStr};

fn main() {
    let is_child = do_clone();
    if is_child {
        entry_point("/bin/hostname", &["hostname"]);
        return(); // Not needed as exec will holt execution but just here for clarity
    }

    println!("I'm here in the parent process");
    return()
}

fn entry_point(program: &str, args: &[&str]) -> () {

    let program_cstring = cstring(program);
    let arg_charptrs = cstring_array(args);


    execv(program_cstring.as_c_str(), arg_charptrs.as_slice()).expect("Exec failed");
}

fn do_clone() -> bool {
    return match fork().expect("Fork Failed") {
        ForkResult::Child => true,
        ForkResult::Parent { child: _ } => false
    }

}

fn cstring(input: &str) -> CString {
    CString::new(input.as_bytes()).unwrap()
}

fn cstring_array<'a>(input: &[&str]) -> Vec<&'a CStr> {
    input.into_iter().map(|&arg| {
        Box::leak(Box::new(CString::new(arg).unwrap())).as_c_str()
    }).collect()
}