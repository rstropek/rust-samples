use std::mem;

/// Print a value byte by byte
#[inline(never)]
pub fn print_bytes<T>(value: &T) {
    let value_as_bytes = unsafe {
        // Create a byte slice from the reference
        std::slice::from_raw_parts(
            value as *const T as *const u8,
            mem::size_of::<T>(),
        )
    };

    print_byte_by_byte(value_as_bytes);
}

/// Print an array byte by byte
#[inline(never)]
pub fn print_array<T, const N: usize>(value: &[T; N]) {
    let value_as_bytes = unsafe {
        // Create a byte slice from the reference
        std::slice::from_raw_parts(
            value as *const T as *const u8,
            N,
        )
    };

    print_byte_by_byte(value_as_bytes);
}

pub fn print_byte_by_byte(value: &[u8]) {
    for byte in value {
        print!("{:02x} ", byte);
    }

    println!();
}