use std::mem;

fn main() {
    let a:u8 = 255;
    println!("Hello {}", a);

    let mut b:i8 = 05;
    b = -5;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification, size = {} bytes", c, mem::size_of_val(&c));

    // i8 u7 i16 u16 i32 u32 i64 u64

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, it takes up {} bytes, {}-bin OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    let dd = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5;
    let ee:f64 = 2.5;
    let eee:f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let f = 4 > 0; // true

}
