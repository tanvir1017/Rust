fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // called early return to immediately close the function work and return
    };

    println!("The {n} is not a even number");
    false // we can write return keyword but it's not required just do not put a semicolon at the end of expression. Yes its called expression
}

fn main() {
    let x = is_even(6);

    println!("{x}");
    just_print();
    println!("The added two number will return `{}`", add_two_num(5, 5));
}


fn just_print(){
    println!("hello this is just return void")
}

fn add_two_num(x: i32, y:i32) -> i32{
    let add: i32 = x + y;
    add
}