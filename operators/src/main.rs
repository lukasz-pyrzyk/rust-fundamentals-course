use std::mem;

fn main() {
    let mut a = 2 + 3 * 4;

    println!("{}", a);
    a = a + 1;
    a -= 1;

    println!("reminder of {} / {} = {}", a, 3, a % 3);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR,
                   // 01 OR 10 == 11 == 3_10

    println!("1|2 = {}", c);

    let two_two_10 = 1 << 10;
    println!("2^10 = {}", two_two_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;

    let x = 5;
    let x_is_5 = x == 5; // true
}
