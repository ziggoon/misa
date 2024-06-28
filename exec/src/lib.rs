use std::ffi::{c_void, c_char, c_int, CStr};
use libc::{getegid};

use config::GID;

#[no_mangle]
pub extern "C" fn execve(path: *const c_char, argv: *const c_char, envp: *const c_char) -> c_int {
    let o_execve: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"execve\0".as_ptr() as *const _) };
    let o_execve: extern "C" fn(*const c_char, *const c_char, *const c_char) -> c_int = unsafe { std::mem::transmute(o_execve) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_execve(path, argv, envp);
    }

    /*
    let program: &str = unsafe { CStr::from_ptr(path).to_str().unwrap() } ;
    println!("{} called", program);

    if program == "iptables" {
        println!("iptables called");
    }
    */

    return o_execve(path, argv, envp);
}
