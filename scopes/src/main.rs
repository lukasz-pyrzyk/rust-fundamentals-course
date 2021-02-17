fn main() {
    let a = 1;
    {
        println!("{}", a);

        let a = 2;
        println!("{}", a);

        let a = 3;
        println!("{}", a);
    }

    println!("{}", a);
}
