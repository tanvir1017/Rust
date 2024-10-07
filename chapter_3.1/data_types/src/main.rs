fn main() {
    

    let a;

    a = 10;
    
    // a = 20; //  will give error because the a is not mutable and you might be think of a = 10 what about that. In there the a = 10 is assigned that's why no error will given

    let a: i8 = 127 ; // Integer data type. Who will not accept any fraction or (.) in number

    let str = "STRING";  // string type.

    let floating_point  = 2.3; // Floating point number. It should be a number with (. -> dot);

    let boolean_type = false; // boolean type

    // Unsigned and Insigned
    let unsigned_number = 250u8; // suffixing  type by adding with value

    let insigne_number = -128i8; // suffixing type by subtracting


    // We can add underscore(der Unterstrich) to visualize and for improve better readability. Like

    let readAble_number = -128_i8; // suffixing type by subtracting

    let readAble_number = 248_u8; // suffixing type by subtracting

    // for best upcases
    let one_cr = 1_00_00_000_u32; // Now its more readable as 1cr
    println!("{}", one_cr);


    // Integer Overflow -> Panic!

    let integer_overflow_example: u8 = 255 + integer_overflow();
    println!("🚀 ~ integer_overflow_example:{}", integer_overflow_example);

    


    //println!("{}", a)
}

/*

// How bits work in RUST :i32 :u8 -> what does it mean

The formula is =>
from -(2n - 1) to ((2n-1) - 1)
[!Link](https://doc.rust-lang.org/book/ch03-02-data-types.html)

by the formula => :i8 means -> -(28-1) to ((28-1) -1)
By the way the n is a power which represent the data type bits

so after calculating -128 to 128 is the value that means we can store value from -128 to 128. and if we go for 129 ti will give us an error

an addition :u8 follow this formula. Because of :u8 doesn't take any negative value
it can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255. 2*2*2*2*2*2*2*2 = 256 -1 => value < 256 that means  from 0 - 255


++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

i32 -> 32-bit signed integer. It can hold values from -2,147,483,648 to 2,147,483,647.

u8 -> 8-bit unsigned integer. It can hold values from 0 to 255.

*/

fn integer_overflow() -> u8 {
    100
}