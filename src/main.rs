fn main() {

    // variables
    let x;
    x=42;

    let x2 = 42;

    // variable with type
    let a:i32 = 42;

    // constant with _ (underscore)
    let _y = 25;
    // _y = 30; // error: cannot assign twice to immutable variable

    // sum
    let sum = x + x2 + a;
    println!("sum: {}", sum);

    println!("Hello, world!");

    // tuples
    let pair = ("a", 17);
    println!("pair 1: {}", pair.0);
    println!("pair 2: {}", pair.1);

    // tuple with type
    let pair2: (char, i32) = ('b', 19);
    println!("pair 1: {}", pair2.0);
    println!("pair 2: {}", pair2.1);

}
