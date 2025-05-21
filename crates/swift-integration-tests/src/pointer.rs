use std::ffi::c_void;
use std::ptr::NonNull;

#[swift_bridge::bridge]
mod ffi {
    // Opaque pointers.

    extern "Rust" {
        fn rust_echo_const_c_void(ptr: *const c_void) -> *const c_void;
        fn rust_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void;
        fn rust_echo_non_null_c_void(ptr: NonNull<c_void>) -> NonNull<c_void>;
        fn rust_echo_optional_non_null_c_void(ptr: Option<NonNull<c_void>>) -> Option<NonNull<c_void>>;

        fn rust_echo_const_u8(ptr: *const u8) -> *const u8;
        fn rust_echo_mut_u8(ptr: *mut u8) -> *mut u8;
        fn rust_echo_non_null_u8(ptr: NonNull<u8>) -> NonNull<u8>;
        fn rust_echo_optional_non_null_u8(ptr: Option<NonNull<u8>>) -> Option<NonNull<u8>>;

        fn rust_run_opaque_pointer_tests();
        fn rust_run_u8_pointer_tests();
    }

    // Opaque pointers.
    extern "Swift" {
        fn swift_echo_const_c_void(ptr: *const c_void) -> *const c_void;
        fn swift_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void;

        fn swift_echo_const_u8(ptr: *const u8) -> *const u8;
        fn swift_echo_mut_u8(ptr: *mut u8) -> *mut u8;

        fn swift_echo_non_null_u8(ptr: NonNull<u8>) -> NonNull<u8>;
        fn swift_echo_non_null_c_void(ptr: NonNull<c_void>) -> NonNull<c_void>;

        fn swift_echo_optional_non_null_u8(ptr: Option<NonNull<u8>>) -> Option<NonNull<u8>>;
        fn swift_echo_optional_non_null_c_void(ptr: Option<NonNull<c_void>>) -> Option<NonNull<c_void>>;
    }
}

/// Verify that we can pass and return opaque pointers across the boundary.
fn rust_run_opaque_pointer_tests() {
    let num = &123;
    let num_mut = &mut 555 as *mut i32;

    let ptr = num as *const i32 as *const c_void;
    let ptr_mut = num_mut as *mut c_void;
    let ptr_non_null = NonNull::new(num_mut as *mut c_void).unwrap();
    let ptr_optional_non_null = NonNull::new(num_mut as *mut c_void);

    let ptr_copy = ffi::swift_echo_const_c_void(ptr);
    let ptr_mut_copy = ffi::swift_echo_mut_c_void(ptr_mut);
    let ptr_non_null_copy = ffi::swift_echo_non_null_c_void(ptr_non_null);
    let ptr_optional_non_null_copy = ffi::swift_echo_optional_non_null_c_void(ptr_optional_non_null);

    let null_void_const = std::ptr::null::<c_void>();
    let null_void_mut = std::ptr::null_mut::<c_void>();
    let null_void_non_null = Option::<NonNull<c_void>>::None;

    let null_void_const_copy = ffi::swift_echo_const_c_void(null_void_const);
    let null_void_mut_copy = ffi::swift_echo_mut_c_void(null_void_mut);
    let null_void_non_null_copy = ffi::swift_echo_optional_non_null_c_void(null_void_non_null);

    assert_eq!(unsafe { *(ptr_copy as *const i32) }, 123);
    assert_eq!(unsafe { *(ptr_mut_copy as *mut i32) }, 555);
    assert_eq!(unsafe { *ptr_non_null_copy.cast::<i32>().as_ref() }, 555);
    assert_eq!(unsafe { *ptr_optional_non_null_copy.unwrap().cast::<i32>().as_mut() }, 555);

    assert_eq!(null_void_const_copy, null_void_const);
    assert_eq!(null_void_mut_copy, null_void_mut);
    assert_eq!(null_void_non_null_copy, null_void_non_null);
}

/// Verify that we can pass and return u8 pointers across the boundary.
fn rust_run_u8_pointer_tests() {
    let num = &5u8;
    let mut num_mut = 10u8;

    let ptr = num as *const u8;
    let ptr_mut = &mut num_mut as *mut u8;
    let ptr_non_null = NonNull::new(&mut num_mut as *mut u8).unwrap();

    let ptr_copy = ffi::swift_echo_const_u8(ptr);
    let ptr_mut_copy = ffi::swift_echo_mut_u8(ptr_mut);
    let ptr_non_null_copy = ffi::swift_echo_non_null_u8(ptr_non_null);

    assert_eq!(unsafe { *ptr_copy }, 5);
    assert_eq!(unsafe { *ptr_mut_copy }, 10);
    assert_eq!(unsafe { *ptr_non_null_copy.as_ref() }, 10);
}

fn rust_echo_const_c_void(ptr: *const c_void) -> *const c_void {
    ptr
}

fn rust_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void {
    ptr
}

fn rust_echo_non_null_c_void(ptr: NonNull<c_void>) -> NonNull<c_void> {
    ptr
}

fn rust_echo_optional_non_null_c_void(ptr: Option<NonNull<c_void>>) -> Option<NonNull<c_void>> {
    ptr
}

fn rust_echo_const_u8(ptr: *const u8) -> *const u8 {
    ptr
}

fn rust_echo_mut_u8(ptr: *mut u8) -> *mut u8 {
    ptr
}

fn rust_echo_non_null_u8(ptr: NonNull<u8>) -> NonNull<u8> {
    ptr
}

fn rust_echo_optional_non_null_u8(ptr: Option<NonNull<u8>>) -> Option<NonNull<u8>> {
    ptr
}

