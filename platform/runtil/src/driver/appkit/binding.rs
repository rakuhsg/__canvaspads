use std::ffi::c_void;

#[repr(C)]
pub struct AppCbs {
    on_init: unsafe extern "C" fn(*const c_void),
    will_deinit: unsafe extern "C" fn(*const c_void),
}

#[repr(C)]
pub struct WindowCbs {
    on_appear: unsafe extern "C" fn(*const c_void),
    will_close: unsafe extern "C" fn(*const c_void),
}

unsafe extern "C" {
    pub fn runtilappkit_init(ud: *const c_void, callback: unsafe extern "C" fn(*const c_void));
    pub fn runtilappkit_schedule();
    pub fn runtilappkit_run();
    pub fn runtilappkit_destroy();
    pub fn runtilappkit_create_window() -> *const c_void;
    pub fn runtilappkit_show_window(ptr: *const c_void);
    pub fn runtilappkit_destroy_window(ptr: *const c_void);
}
