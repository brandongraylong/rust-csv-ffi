// TODO: Remove these
// #![allow(dead_code)]
// #![allow(unused_variables)]

extern crate libc;

use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::ptr::{null_mut};
use std::{ptr, mem, usize};
use csv::CSV;

#[no_mangle]
pub extern "C" fn csv_new() -> *mut CSV {
    return Box::into_raw(Box::new(CSV::new()));
}

#[no_mangle]
pub extern "C" fn csv_free(ptr: *mut CSV) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn csv_rows(ptr: *mut CSV) -> size_t {
    if ptr.is_null() {
        return 0;
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    
    return csv.rows();
}

#[no_mangle]
pub extern "C" fn csv_columns(ptr: *mut CSV) -> size_t {
    if ptr.is_null() {
        return 0;
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    return csv.columns();
}

#[no_mangle]
pub extern "C" fn csv_read(ptr: *mut CSV, path: *const c_char) {
    let path_c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };

    let path_r_str = path_c_str.to_str().unwrap().to_string();
    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    csv.read(path_r_str);
}

#[no_mangle]
pub extern "C" fn csv_write(ptr: *mut CSV, path: *const c_char) {
    let path_c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };

    let path_r_str = path_c_str.to_str().unwrap().to_string();
    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    csv.write(path_r_str);
}

#[no_mangle]
pub extern "C" fn csv_get_headers(ptr: *mut CSV, outlen: *mut size_t) -> *mut *mut c_char {
    if ptr.is_null() {
        return null_mut();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let mut out: Vec<*mut c_char> = csv.data().keys().iter().map(|val| CString::new(val.to_owned()).unwrap().into_raw()).collect::<Vec<_>>();
    out.shrink_to_fit();
    assert!(out.len() == out.capacity());
    let len: usize = out.len();
    let ptr: *mut *mut c_char = out.as_mut_ptr();
    mem::forget(out);
    unsafe {
        ptr::write(outlen, len as size_t);
    }
    
    return ptr;
}

#[no_mangle]
pub extern "C" fn csv_get_row(ptr: *mut CSV, row: size_t, outlen: *mut size_t) -> *mut *mut c_char {
    if ptr.is_null() {
        return null_mut();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let mut out: Vec<*mut c_char> = {
        if row < csv.data().len() {
            let mut v: Vec<*mut c_char> = Vec::new();
            for i in 0..csv.columns() {
                v.push(CString::new(csv.data().at(i).unwrap().get(row).unwrap().to_owned()).unwrap().into_raw())
            }
            v
        } else {
            Vec::new()
        }
    };
    out.shrink_to_fit();
    assert!(out.len() == out.capacity());
    let len: usize = out.len();
    let ptr: *mut *mut c_char = out.as_mut_ptr();
    mem::forget(out);
    unsafe {
        ptr::write(outlen, len as size_t);
    }
    
    return ptr;
}

#[no_mangle]
pub extern "C" fn csv_get_column(ptr: *mut CSV, key: *const c_char, outlen: *mut size_t) -> *mut *mut c_char {
    if ptr.is_null() {
        return null_mut();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let key_c_str = unsafe {
        assert!(!key.is_null());

        CStr::from_ptr(key)
    };
    let key_r_string = key_c_str.to_str().unwrap().to_string();

    let mut out: Vec<*mut c_char> = {
        if csv.data().contains_key(key_r_string.to_owned()) {
            csv.data().get(key_r_string).unwrap().iter().map(|val| CString::new(val.to_owned()).unwrap().into_raw()).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    };
    out.shrink_to_fit();
    assert!(out.len() == out.capacity());
    let len: usize = out.len();
    let ptr: *mut *mut c_char = out.as_mut_ptr();
    mem::forget(out);
    unsafe {
        ptr::write(outlen, len as size_t);
    }
    
    return ptr;
}

#[no_mangle]
pub extern "C" fn csv_print(ptr: *const CSV) {
    let csv = unsafe {
        assert!(!ptr.is_null());
        & *ptr
    };

    csv.print();
}
