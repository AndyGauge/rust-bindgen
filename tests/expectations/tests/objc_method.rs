/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo {
    unsafe fn method(self);
    unsafe fn methodWithInt_(self, foo: ::std::os::raw::c_int);
    unsafe fn methodWithFoo_(self, foo: id);
    unsafe fn methodReturningInt(self) -> ::std::os::raw::c_int;
    unsafe fn methodReturningFoo(self) -> *mut id;
    unsafe fn methodWithArg1_andArg2_andArg3_(
        self,
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    );
}
impl Foo for id {
    unsafe fn method(self) {
        msg_send!(self, method)
    }
    unsafe fn methodWithInt_(self, foo: ::std::os::raw::c_int) {
        msg_send!(self, methodWithInt: foo: ::std::os::raw::c_int)
    }
    unsafe fn methodWithFoo_(self, foo: id) {
        msg_send!(self, methodWithFoo: foo: id)
    }
    unsafe fn methodReturningInt(self) -> ::std::os::raw::c_int {
        msg_send!(self, methodReturningInt)
    }
    unsafe fn methodReturningFoo(self) -> *mut id {
        msg_send!(self, methodReturningFoo)
    }
    unsafe fn methodWithArg1_andArg2_andArg3_(
        self,
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    ) {
        msg_send!(
            self,
            methodWithArg1: intvalue: ::std::os::raw::c_int,
            andArg2: ptr: *mut ::std::os::raw::c_char,
            andArg3: floatvalue: f32
        )
    }
}
