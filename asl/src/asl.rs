use std::mem;

mod sys {
    use super::{Address, PointerKind, State};

    extern "C" {
        pub fn set_process_name(name_ptr: *const u8, name_len: usize);
        pub fn push_pointer_path(
            module_ptr: *const u8,
            module_len: usize,
            kind: PointerKind,
        ) -> usize;
        pub fn pop_pointer_path(pointer_path_id: usize);
        pub fn push_offset(pointer_path_id: usize, offset: i64);
        pub fn get_u8(pointer_path_id: usize, current: State) -> u8;
        pub fn get_u16(pointer_path_id: usize, current: State) -> u16;
        pub fn get_u32(pointer_path_id: usize, current: State) -> u32;
        pub fn get_u64(pointer_path_id: usize, current: State) -> u64;
        pub fn get_i8(pointer_path_id: usize, current: State) -> i8;
        pub fn get_i16(pointer_path_id: usize, current: State) -> i16;
        pub fn get_i32(pointer_path_id: usize, current: State) -> i32;
        pub fn get_i64(pointer_path_id: usize, current: State) -> i64;
        pub fn get_f32(pointer_path_id: usize, current: State) -> f32;
        pub fn get_f64(pointer_path_id: usize, current: State) -> f64;
        pub fn scan_signature(sig_ptr: *const u8, sig_len: usize) -> Address;
        pub fn set_tick_rate(rate: f64);
        pub fn print_message(text_ptr: *const u8, text_len: usize);
        pub fn read_into_buf(address: Address, buf: *mut u8, buf_len: usize) -> u8;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Address(pub u64);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PointerKind {
    U8 = 0,
    U16 = 1,
    U32 = 2,
    U64 = 3,
    I8 = 4,
    I16 = 5,
    I32 = 6,
    I64 = 7,
    F32 = 8,
    F64 = 9,
    String = 10,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    Old,
    Current,
}

pub fn set_process_name(module: &str) {
    unsafe {
        sys::set_process_name(module.as_ptr() as *const u8, module.len());
    }
}

pub fn push_pointer_path(module: &str, offsets: &[i64], kind: PointerKind) {
    unsafe {
        let id = sys::push_pointer_path(module.as_ptr() as *const u8, module.len(), kind);
        for &offset in offsets {
            sys::push_offset(id, offset);
        }
    }
}

pub fn pop_pointer_path(pointer_path_id: usize) {
    unsafe { sys::pop_pointer_path(pointer_path_id) }
}

pub fn get_u8(pointer_path_id: usize, current: State) -> u8 {
    unsafe { sys::get_u8(pointer_path_id, current) }
}

pub fn get_u16(pointer_path_id: usize, current: State) -> u16 {
    unsafe { sys::get_u16(pointer_path_id, current) }
}

pub fn get_u32(pointer_path_id: usize, current: State) -> u32 {
    unsafe { sys::get_u32(pointer_path_id, current) }
}

pub fn get_u64(pointer_path_id: usize, current: State) -> u64 {
    unsafe { sys::get_u64(pointer_path_id, current) }
}

pub fn get_i8(pointer_path_id: usize, current: State) -> i8 {
    unsafe { sys::get_i8(pointer_path_id, current) }
}

pub fn get_i16(pointer_path_id: usize, current: State) -> i16 {
    unsafe { sys::get_i16(pointer_path_id, current) }
}

pub fn get_i32(pointer_path_id: usize, current: State) -> i32 {
    unsafe { sys::get_i32(pointer_path_id, current) }
}

pub fn get_i64(pointer_path_id: usize, current: State) -> i64 {
    unsafe { sys::get_i64(pointer_path_id, current) }
}

pub fn get_f32(pointer_path_id: usize, current: State) -> f32 {
    unsafe { sys::get_f32(pointer_path_id, current) }
}

pub fn get_f64(pointer_path_id: usize, current: State) -> f64 {
    unsafe { sys::get_f64(pointer_path_id, current) }
}

pub fn scan_signature(signature: &str) -> Option<Address> {
    let address = unsafe { sys::scan_signature(signature.as_ptr(), signature.len()) };
    if address.0 == 0 {
        None
    } else {
        Some(address)
    }
}

pub fn set_tick_rate(rate: f64) {
    unsafe { sys::set_tick_rate(rate) }
}

pub fn print_message(text: &str) {
    unsafe { sys::print_message(text.as_ptr(), text.len()) }
}

pub fn read_into_buf(address: Address, buf: &mut [u8]) -> Result<(),()> {
    let res = unsafe { sys::read_into_buf(address, buf.as_mut_ptr(), buf.len()) };
    if res != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub unsafe fn read_val<T>(address: Address) -> Result<T, ()> {
    let mut val = mem::uninitialized();
    let res = sys::read_into_buf(
        address,
        (&mut val) as *mut T as *mut u8,
        mem::size_of::<T>(),
    );
    if res != 0 {
        Ok(val)
    } else {
        Err(())
    }
}

pub trait ASLState
where
    Self: Sized,
{
    fn get() -> (Self, Self);
}
