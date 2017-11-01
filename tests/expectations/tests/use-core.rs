/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate core;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub bar: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::core::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::core::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).b as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bar as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _bindgen_ty_1 {
    pub bar: ::std::os::raw::c_int,
    pub baz: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(0 as *const _bindgen_ty_1)).bar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(bar)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const _bindgen_ty_1)).baz as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(baz)
        )
    );
}
impl Default for _bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    #[link_name = "\u{1}bazz"]
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction = ::core::option::Option<unsafe extern "C" fn(bar: ::std::os::raw::c_int)>;
