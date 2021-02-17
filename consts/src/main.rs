const MEANING_OF_LIFE:u8 = 42;

static Z:i32 = 42;
static mut ZMut:i32 = 123;

fn main() {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", Z);
    unsafe
    {
        println!("{}", ZMut)
    }
}
