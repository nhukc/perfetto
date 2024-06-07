extern crate libc;

use libc::c_void;
use std::ffi::CString;

extern "C" {
    fn initialize_tracing();
    fn create_observer() -> *mut c_void;
    fn wait_for_tracing_start(observer: *mut c_void);
    fn wait();
    fn flush_tracing();
    fn destroy_observer(observer: *mut c_void);
}

pub fn rust_initialize_tracing() {
    unsafe {
        initialize_tracing();
    }
}

pub fn rust_create_observer() -> *mut c_void {
    unsafe {
        create_observer()
    }
}

pub fn rust_wait_for_tracing_start(observer: *mut c_void) {
    unsafe {
        wait_for_tracing_start(observer);
    }
}

pub fn rust_wait() {
    unsafe {
        wait();
    }
}

pub fn rust_flush_tracing() {
    unsafe {
        flush_tracing();
    }
}

pub fn rust_destroy_observer(observer: *mut c_void) {
    unsafe {
        destroy_observer(observer);
    }
}
