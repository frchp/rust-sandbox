fn main() {
    println!("Hello, world!");

    numbers();
}

fn numbers() {
    // let val1: i32 = 4;
    // let val2 = 5;
    // let val3 = 6usize;
    // let val_archi = val1 as usize + val2 as usize;
    // let mut val_archi = val3 + val2 as usize;
    // println!("Numbers {} {val_archi}", val_archi);

    let mut a = 5;
    a = 6;

    // error[E0384]: cannot assign twice to immutable variable `a`
    let a = 5;
    a = 6;

    println!("Numbers {} {a}", a);
}