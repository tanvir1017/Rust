fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // called early return to immediately close the function work and return
    };

    println!("The {n} is not a even number");

    false
}

fn main() {
    let x = is_even(6);

    println!("{x}")
}
