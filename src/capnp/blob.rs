/*
 * Copyright (c) 2013-2014, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

pub mod text {
    use std;

    pub type Reader<'a> = &'a str;

    static EMPTY : &'static str = "";

    // len does not include the required null terminator at the end
    pub fn new_reader<'a>(p : *const u8, len : u32) -> Option<Reader<'a>> {
        // XXX The empty case is special and I don't know why.
        if len == 0 { return Some(EMPTY); }
        let v : &'a [u8] =
            unsafe { std::mem::transmute(std::raw::Slice { data: p, len: len as uint}) };
        std::str::from_utf8(v)
    }

    pub struct Builder<'a> {
        ptr : *mut u8,
        len : uint,
    }

    impl <'a> Builder <'a> {

        pub fn new<'b>(p : *mut u8, len : u32) -> Builder<'b> {
            Builder { ptr : p, len : len as uint}
        }

        pub fn as_mut_bytes(&self) -> &'a mut [u8] {
             unsafe { std::mem::transmute(std::raw::Slice { data:self.ptr as *const u8, len: self.len }) }
        }

        pub fn as_ptr(&self) -> *mut u8 {
            self.ptr
        }
    }

}

pub mod data {
    use std;

    pub type Reader<'a> = &'a [u8];

    pub fn new_reader<'a>(p : *const u8, len : u32) -> Reader<'a> {
        unsafe {
            let v = std::raw::Slice { data: p, len: len as uint};
            std::mem::transmute(v)
        }
    }

    pub type Builder<'a> = &'a mut [u8];

    pub fn new_builder<'a>(p : *mut u8, len : u32) -> Builder<'a> {
        unsafe {
            let v = std::raw::Slice { data: p as *const u8, len: len as uint};
            std::mem::transmute(v)
        }
    }

}
