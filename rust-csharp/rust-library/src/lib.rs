use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub extern "C" fn add_vec2d(left: Vec2d, right: Vec2d) -> Vec2d {
    Vec2d {
        X: left.X + right.X,
        Y: left.Y + right.Y,
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn strlen(text: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!text.is_null());

        CStr::from_ptr(text)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Vec2d {
    X: f32,
    Y: f32,
}
