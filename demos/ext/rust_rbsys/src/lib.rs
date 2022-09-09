use std::ffi::CStr;

use rb_sys::{
    rb_define_global_function, rb_eArgError, rb_enc_str_new, rb_raise, rb_string_value_cstr,
    rb_utf8_encoding, Qnil, VALUE,
};

unsafe extern "C" fn reverse(_klass: VALUE, mut input: VALUE) -> VALUE {
    let input_cstr = rb_string_value_cstr(&mut input as _);
    let input_str = CStr::from_ptr(input_cstr);

    match input_str.to_str() {
        Ok(input) => {
            let result: String = input.chars().rev().collect();
            rb_enc_str_new(result.as_ptr() as _, result.len() as _, rb_utf8_encoding())
        }
        Err(_) => {
            rb_raise(rb_eArgError, "invalid UTF-8\0".as_ptr() as _);
            Qnil as _
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Init_rust_rbsys() {
    let reverse_func = std::mem::transmute::<
        unsafe extern "C" fn(VALUE, VALUE) -> VALUE,
        unsafe extern "C" fn() -> VALUE,
    >(reverse);

    rb_define_global_function(
        "reverse_it_rust_rbsys\0".as_ptr() as _,
        Some(reverse_func),
        1,
    );
}
