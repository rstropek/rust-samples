#![allow(dead_code)]

use lifetimes::characters_with_stuff;
use lor::ownership_ring;
use lor_rc::{share_and_alter, share_the_ring};
use print::{print_bytes, print_array};

mod print;
mod lor;
mod lor_rc;
mod lifetimes;

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone)]
struct PointCopy {
    x: i32,
    y: i32,
}


fn main() {
    // just asm ...

    primitive_data_type_on_stack();
    array_on_stack();
    struct_on_stack();
    heap_box();
    ownership();
    copy_trait();

    ownership_ring();

    characters_with_stuff();

    share_the_ring();
    share_and_alter();
}

#[inline(never)]
pub fn primitive_data_type_on_stack() {
    let mut x = 42;
    x += 1;
    // 43 is 0x2b in hex

    print_bytes(&x);
}

#[inline(never)]
pub fn array_on_stack() {
    let numbers = [0x68, 0x69, 0x0a, 0];
    // 1819043176 is 0x6c6c6568 in hex
    // 2671 is 0x0a6f in hex
    
    print_array(&numbers);
}

#[inline(never)]
pub fn struct_on_stack() {
    let mut point = Point3d { x: 15, y: 14, z: 13 };
    point.x += 1;
    point.y += 2;
    point.z += 3;
    // 68719476752 is 0x10 00 00 00 10 in hex

    print_bytes(&point);
}

#[inline(never)]
pub fn heap_box() {
    let mut point = Box::new(Point { x: 15, y: 14 });
    point.x += 1;
    point.y += 2;
    // __rust_allow has to arguments: size and alignment
    // edi is used for size -> point is on the heap
    // rbx contains pointer to point struct
    
    print_bytes(point.as_ref());
    
    // __rust_dealloc has three arguments: pointer, size and alignment
    // rbx contains pointer to point struct
}

#[inline(never)]
pub fn ownership() {
    let mut point = Point { x: 15, y: 14 };
    point.x += 1;
    point.y += 2;
    
    // Let's see what happens when we transfer ownership
    let point2 = point;
    // NOTHING!

    print_bytes(&point2);
}

#[inline(never)]
pub fn copy_trait() {
    let mut point = PointCopy { x: 14, y: 12 };
    point.x += 1;
    point.y += 2;
    // 60129542159 is 0x0e 00 00 00 0f in hex
    let mut point2 = point;
    point2.x += 1;
    point2.y += 2;
    // 68719476752 is 0x10 00 00 00 10 in hex

    print_bytes(&point);
    print_bytes(&point2);
}
