#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn exit(__status: core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn fatal(mut message: *const core::ffi::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const core::ffi::c_char, message);
    exit(1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut core::ffi::c_void {
    let mut ptr: *mut core::ffi::c_void = malloc(n);
    if ptr.is_null() {
        fatal(b"Out of memory\0" as *const u8 as *const core::ffi::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn icompare(
    mut p1: *const core::ffi::c_void,
    mut p2: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let mut ip1: *const core::ffi::c_int = p1 as *const core::ffi::c_int;
    let mut ip2: *const core::ffi::c_int = p2 as *const core::ffi::c_int;
    return if *ip1 < *ip2 {
        -(1 as core::ffi::c_int)
    } else if *ip1 > *ip2 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn unique(
    mut array: *mut core::ffi::c_int,
    mut len: size_t,
) -> size_t {
    let mut out_index: size_t = 0 as size_t;
    let mut prev: core::ffi::c_int = 0;
    let mut i: size_t = 0 as size_t;
    while i < len {
        if i == 0 as size_t || prev != *array.offset(i as isize) {
            let fresh0 = out_index;
            out_index = out_index.wrapping_add(1);
            *array.offset(fresh0 as isize) = *array.offset(i as isize);
        }
        prev = *array.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return out_index;
}
#[no_mangle]
pub unsafe extern "C" fn common_sorted_list(
    mut arrays: *mut *const core::ffi::c_int,
    mut lengths: *const size_t,
    mut count: size_t,
    mut size: *mut size_t,
) -> *mut core::ffi::c_int {
    let mut len: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < count {
        len = len.wrapping_add(*lengths.offset(i as isize));
        i = i.wrapping_add(1);
    }
    let mut array: *mut core::ffi::c_int = xmalloc(
        len.wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) as *mut core::ffi::c_int;
    let mut i_0: size_t = 0 as size_t;
    let mut offset: size_t = 0 as size_t;
    while i_0 < count {
        memcpy(
            array.offset(offset as isize) as *mut core::ffi::c_void,
            *arrays.offset(i_0 as isize) as *const core::ffi::c_void,
            (*lengths.offset(i_0 as isize))
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        offset = offset.wrapping_add(*lengths.offset(i_0 as isize));
        i_0 = i_0.wrapping_add(1);
    }
    qsort(
        array as *mut core::ffi::c_void,
        len,
        ::core::mem::size_of::<core::ffi::c_int>() as size_t,
        Some(
            icompare
                as unsafe extern "C" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    *size = unique(array, len);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn print(mut array: *const core::ffi::c_int, mut len: size_t) {
    printf(b"[\0" as *const u8 as *const core::ffi::c_char);
    let mut i: size_t = 0 as size_t;
    while i < len {
        if i > 0 as size_t {
            printf(b", \0" as *const u8 as *const core::ffi::c_char);
        }
        printf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            *array.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    printf(b"]\n\0" as *const u8 as *const core::ffi::c_char);
}
